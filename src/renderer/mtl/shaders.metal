#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct Vertex {
    float2 pos [[attribute(0)]];
    float2 tcoord [[attribute(1)]];
};

struct RasterizerData {
    float4 pos [[position]];
    float2 fpos;
    float2 ftcoord;
};




// float scissorMat[12]; // matrices are actually 3 vec4s
// float paintMat[12];
// struct NVGcolor innerCol;
// struct NVGcolor outerCol;
// float scissorExt[2];
// float scissorScale[2];
// float extent[2];
// float radius;
// float feather;
// float strokeMult;
// float strokeThr;
// float texType;
// float type;

#define STATIC_ASSERT(COND,MSG) typedef char static_assertion_##MSG[(!!(COND))*2-1]
#define COMPILE_TIME_ASSERT3(X,L) STATIC_ASSERT(X,static_assertion_at_line_##L)
#define COMPILE_TIME_ASSERT2(X,L) COMPILE_TIME_ASSERT3(X,L)
#define COMPILE_TIME_ASSERT(X)    COMPILE_TIME_ASSERT2(X,__LINE__)

enum ShaderType {
    FillGradient,
    FillImage,
    Stencil,
};

// COMPILE_TIME_ASSERT(sizeof(Uniforms) == 4);

struct Uniforms {
    float3x4 scissorMat;
    float3x4 paintMat;
    float4 innerCol;
    float4 outerCol;
    float2 scissorExt;
    float2 scissorScale;
    float2 extent;
    float radius;
    float feather;
    float strokeMult;
    float strokeThr;
    float texType;
    float shaderType;
    float hasMask;
    float padding[19];
};

COMPILE_TIME_ASSERT(sizeof(Uniforms) == 256);

float scissorMask(constant Uniforms& uniforms, float2 p);
float sdroundrect(constant Uniforms& uniforms, float2 pt);
float strokeMask(constant Uniforms& uniforms, float2 ftcoord);

float scissorMask(constant Uniforms& uniforms, float2 p) {
    float2 sc = (abs((uniforms.scissorMat * float3(p, 1.0f)).xy)
                 - uniforms.scissorExt);
    
    sc = float2(0.5f) - sc * uniforms.scissorScale;
    // clamp
    // return sc.x * sc.y;
    return clamp(sc.x, 0.0, 1.0) * clamp(sc.y, 0.0, 1.0);
}

float sdroundrect(constant Uniforms& uniforms, float2 pt) {
    float2 ext2 = uniforms.extent - float2(uniforms.radius);
    float2 d = abs(pt) - ext2;
    return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - uniforms.radius;
}
// float sdroundrect2(float2 pt, float2 ext, float rad) {
//     float2 ext2 = ext - float2(rad);
//     float2 d = abs(pt) - ext2;
//     return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - rad;
// }


float strokeMask(constant Uniforms& uniforms, float2 ftcoord) {
    return min(1.0, (1.0 - abs(ftcoord.x * 2.0 - 1.0)) * uniforms.strokeMult) \
    * min(1.0, ftcoord.y);
}

// Vertex Function
vertex RasterizerData vertexShader(Vertex vert [[stage_in]],
                                   constant float2& viewSize [[buffer(1)]]) {
    RasterizerData out;
    out.ftcoord = vert.tcoord;
    out.fpos = vert.pos;
    out.pos = float4(2.0 * vert.pos.x / viewSize.x - 1.0,
                     1.0 - 2.0 * vert.pos.y / viewSize.y,
                     0, 1);
    return out;
}

// Fragment function (No AA)
// fragment float4 fragmentShader(
//   RasterizerData in [[stage_in]],
//   constant Uniforms& uniforms [[buffer(0)]],
//   texture2d<float> texture [[texture(0)]],
//   sampler samplr [[sampler(0)]],
//   texture2d<float> alpha_texture [[texture(1)]],
//   sampler alpha_samplr [[sampler(1)]]
// ) {
//     float scissor = scissorMask(uniforms, in.fpos);
//     if (scissor == 0) {
//         return float4(0);
//     }

//     float4 result;

//     if (uniforms.shaderType == 0) {
//         // MNVG_SHADER_FILLGRAD
//         float2 pt = (uniforms.paintMat * float3(in.fpos, 1.0)).xy;
//         float d = saturate((uniforms.feather * 0.5 + sdroundrect(uniforms, pt))
//                            / uniforms.feather);
//         float4 color = mix(uniforms.innerCol, uniforms.outerCol, d);
//         result = color * scissor;
//     } else if (uniforms.shaderType == 1) {
//         // MNVG_SHADER_FILLIMG
//         float2 pt = (uniforms.paintMat * float3(in.fpos, 1.0)).xy / uniforms.extent;
//         float4 color = texture.sample(samplr, pt);
//         if (uniforms.texType == 1) {
//             color = float4(color.xyz * color.w, color.w);
//         }
//         else if (uniforms.texType == 2) {
//             color = float4(color.x);
//         }
//         color *= scissor;
//         result = color * uniforms.innerCol;
//     } else {
//         // MNVG_SHADER_IMG
//         float4 color = texture.sample(samplr, in.ftcoord);
//         if (uniforms.texType == 1) {
//             color = float4(color.xyz * color.w, color.w);
//         }
//         else if (uniforms.texType == 2) {
//             color = float4(color.x);
//         }
//         color *= scissor;
//         result = color * uniforms.innerCol;
//     }

//     return result;
// }
fragment float4 fragmentShaderAA(
    RasterizerData in [[stage_in]],
    constant Uniforms& uniforms [[buffer(0)]],
    texture2d<float> texture [[texture(0)]],
    sampler samplr [[sampler(0)]],
    texture2d<float> alpha_texture [[texture(1)]],
    sampler alpha_samplr [[sampler(1)]]
) {
    float4 result;
    float scissor = scissorMask(uniforms, in.fpos);
    
    float strokeAlpha = strokeMask(uniforms, in.ftcoord);
    if (strokeAlpha < uniforms.strokeThr) {
        // result = float4(0);
        discard_fragment();
    }

    // if (scissor == 0) {
    //     return float4(0);
    // }

// Self::FillGradient => 0.0,
// Self::FillImage => 1.0,
// Self::Stencil => 2.0,
    if (uniforms.shaderType == 0) {
        // MNVG_SHADER_FILLGRAD
        float2 pt = (uniforms.paintMat * float3(in.fpos, 1.0)).xy;
        // revisit d
        float d = clamp((sdroundrect(uniforms, pt) + uniforms.feather*0.5) / uniforms.feather, 0.0, 1.0);
        // float d = saturate((uniforms.feather * 0.5 + sdroundrect(uniforms, pt))
        //                    / uniforms.feather);
        float4 color = mix(uniforms.innerCol, uniforms.outerCol, d);
        // color *= scissor;
        // color *= strokeAlpha;
        result = color;
    } else if (uniforms.shaderType == 1) {
        // MNVG_SHADER_IMG
        // revisit: should this be ftcoord or fpos or the other one?
        float2 pt = (uniforms.paintMat * float3(in.ftcoord, 1.0)).xy / uniforms.extent;
        // float4 color = texture.sample(samplr, in.ftcoord);
        
        float4 color = texture.sample(samplr, pt);
        if (uniforms.texType == 1) {
            color = float4(color.xyz * color.w, color.w);
        }
        else if (uniforms.texType == 2) {
            color = float4(color.x);
        }
        // color *= scissor;
        result = color * scissor * uniforms.innerCol;
    } else {
        // stencil
        // MNVG_SHADER_FILLIMG
        result = float4(1.0);
        // float2 pt = (uniforms.paintMat * float3(in.fpos, 1.0)).xy / uniforms.extent;
        // float4 color = texture.sample(samplr, pt);
        // if (uniforms.texType == 1) {
        //     color = float4(color.xyz * color.w, color.w);
        // }
        // else if (uniforms.texType == 2) {
        //     color = float4(color.x);
        // }
        // color *= scissor;
        // color *= strokeAlpha;
        // result = color * uniforms.innerCol;
    }

    if (uniforms.hasMask == 1.0) {
        // revisit ftcoord
        float2 ftcoord = float2(in.ftcoord.x, 1.0 - in.ftcoord.y);
        float4 mask = float4(alpha_texture.sample(alpha_samplr, ftcoord).r);

        // if (alpha < uniforms.strokeThr) {
        //     discard_fragment();
        // }
    //     // result /= strokeAlpha;
    //     result /= 0.02;
        mask *= scissor;
        result *= mask;
        // result = float4(result.xyz, alpha);

        // return float4(0.5, 0.5, 0.5, 0.3);
    //     //  float r = alpha_texture.sample(alpha_samplr, in.ftcoord).x;
    //     //  float4 smpl = vec4(1.0, 1.0, 1.0, smpl);
    //     //  result = vec4(result.xyz, 1.0) * smpl;
    //     // if(type.type == TypeText) {
	// 	//     out_color.a *= texture(font, in_uv).a;
	//     // }
    //     // result.a *= alpha_texture.sample(alpha_samplr, in.ftcoord).a;

    //     // float4 smpl = alpha_texture.sample(alpha_samplr, in.ftcoord);
    //     // float4 mask = float4(smpl.x);
    //     // result *= smpl;
    //     // return float4(1.0);

    
    //     // else {
    //         // return float4(result.xyz, smpl.a);
    //     // }
    }
    else if (uniforms.shaderType != 2.0) {
        result *= strokeAlpha * scissor;
    }

    return result;
}


// Fragment function (AA)
fragment float4 fragmentShaderAAUpdated(
    RasterizerData in [[stage_in]],
    constant Uniforms& u [[buffer(0)]],
    texture2d<float> texture [[texture(0)]],
    sampler samplr [[sampler(0)]],
    texture2d<float> alpha_texture [[texture(1)]],
    sampler alpha_samplr [[sampler(1)]]
) {
    float4 result;
    float scissor = scissorMask(u, in.fpos);
    // if (scissor == 0) {
    //     return float4(0);
    // }

    float strokeAlpha = strokeMask(u, in.ftcoord);
    if (strokeAlpha < u.strokeThr) {
        discard_fragment();
    }

    if (u.shaderType == 0) {
        // MNVG_SHADER_FILLGRAD
        // gradient
        // float2 pt = (u.paintMat * float3(in.fpos, 1.0)).xy;
        // float d = clamp((sdroundrect2(pt, u.extent, u.radius) + u.feather*0.5) / u.feather, 0.0, 1.0);

        // float4 color = mix(u.innerCol, u.outerCol, d);
        // result = color;
    } else if (u.shaderType == 1) {
        // MNVG_SHADER_FILLIMG
        // image
        float2 pt = (u.paintMat * float3(in.fpos, 1.0)).xy / u.extent;
        float4 color = texture.sample(samplr, pt);
        if (u.texType == 1) {
            color = float4(color.xyz * color.w, color.w);
        }
        else if (u.texType == 2) {
            color = float4(color.x);
        }
        // color *= scissor;
        // color *= strokeAlpha;
        result = color * u.innerCol;
    }
    else {
        // MNVG_SHADER_IMG
        // stencil
        float4 color = texture.sample(samplr, in.ftcoord);
        if (u.texType == 1) {
            color = float4(color.xyz * color.w, color.w);
        }
        else if (u.texType == 2) {
            color = float4(color.x);
        }
        color *= scissor;
        result = color * u.innerCol;
        // result = float4(1.0);
    }

    if (u.hasMask == 1.0) {

        // this is from vulkan cpp nanovg impl
        //  float4 mask = alpha_texture.sample(alpha_samplr, in.ftcoord);
        //  result.a *= mask.a;

        float alpha = alpha_texture.sample(alpha_samplr, in.ftcoord).r;

        // if (alpha <= 0.0) {
        //     discard_fragment();
        // }

//         half4 baseColor = baseTexture.sample(colorSampler, in.texCoords);
//   half4 alphaColor = alphaTexture.sample(colorSampler, in.texCoords);
//   return half4(baseColor.r, baseColor.g, baseColor.b, alphaColor.r);
        result = float4(result.xyz, alpha);
        // return vert.color * float4(1.0, 1.0, 1.0, alpha);

        //  mask = float4(mask.x);
        //  mask *= scissor;
        // mask *= scissor;
        // result *= mask;

    //     if(type.type == TypeText) {
	// 	out_color.a *= texture(font, in_uv).a;
	// }

        //  result /= strokeAlpha;
        //  result = float4(result.rgb * r, r);
        //  return float4(1.0);
        //  float4 smpl = vec4(1.0, 1.0, 1.0, smpl);
        //  result = vec4(result.xyz, 1.0) * smpl;
        // if(type.type == TypeText) {
		//     out_color.a *= texture(font, in_uv).a;
	    // }
        // result.a *= alpha_texture.sample(alpha_samplr, in.ftcoord).a;

        // float4 smpl = alpha_texture.sample(alpha_samplr, in.ftcoord);
        // float4 mask = float4(smpl.x);
        // result *= smpl;
        // return float4(1.0);

        // if (smpl.a < 1.1) {
        //     discard_fragment();
        // }
        // else {
            // return float4(result.xyz, smpl.a);
        // }
    }
    else if (u.shaderType != 2.0) {
        result *= strokeAlpha * scissor;
    }

    return result;
}

struct ColorInOut {
    float4 position [[position]];
    float4 color;
};

struct Rect {
    float x;
    float y;
    float w;
    float h;
};

struct Color {
    float r;
    float g;
    float b;
    float a;
};

struct ClearRect {
    Rect rect;
    Color color;
};

float2 rect_vert_cw(
    Rect rect,
    uint vid
) {
    float2 pos;

    float left = rect.x;
    float right = rect.x + rect.w;
    float bottom = rect.y;
    float top = rect.y + rect.h;

    switch (vid) {
    case 0:
        pos = float2(right, top);
        break;
    case 1:
        pos = float2(left, top);
        break;
    case 2:
        pos = float2(right, bottom);
        break;
    case 3:
        pos = float2(left, bottom);
        break;
    }
    return pos;
}

/// gets the vertices in counterclockwise order
/// so that this plays nicely with the cull mode set
/// to flip to clockwise the flip 1 and 2
float2 rect_vert_ccw(
    Rect rect,
    uint vid
) {
    float2 pos;

    float left = rect.x;
    float right = rect.x + rect.w;
    float bottom = rect.y;
    float top = rect.y + rect.h;

    switch (vid) {
    case 0:
        pos = float2(right, top);
        break;
    case 1:
        pos = float2(right, bottom);
        break;
    case 2:
        pos = float2(left, top);
        break;
    case 3:
        pos = float2(left, bottom);
        break;
    }
    return pos;
}

vertex ColorInOut clear_rect_vertex(
    const device ClearRect *clear_rect [[ buffer(0) ]],
    unsigned int vid [[ vertex_id ]]
) {
    ColorInOut out;
    float4 pos = float4(rect_vert_cw(clear_rect->rect, vid), 0, 1);
    auto col = clear_rect->color;
    // float4x4 tform = float4x4(1,0,0,0,
    //                           0,-1,0,0,
    //                           0,0,1,0,
    //                           0,0,0,1);
    out.position = pos;
    out.color = float4(col.r, col.g, col.b, col.a);
    return out;
}

fragment float4 clear_rect_fragment(ColorInOut in [[stage_in]]) {
    return in.color;
};


// typedef struct {
// 	packed_float2 position;
// 	packed_float3 color;
// } vertex_t;

// // vertex shader function
// vertex ColorInOut triangle_vertex(const device vertex_t* vertex_array [[ buffer(0) ]],
//                                    unsigned int vid [[ vertex_id ]])
// {
//     ColorInOut out;

//     auto device const &v = vertex_array[vid];
//     out.position = float4(v.position.x, v.position.y, 0.0, 1.0);
//     out.color = float4(v.color.x, v.color.y, v.color.z, 0.2);

//     return out;
// }

// // fragment shader function
// fragment float4 triangle_fragment(ColorInOut in [[stage_in]])
// {
//     return in.color;
// };

