
precision mediump float;

#define UNIFORMARRAY_SIZE 12

uniform vec4 frag[UNIFORMARRAY_SIZE];

#define scissorMat mat3(frag[0].xyz, frag[1].xyz, frag[2].xyz)
#define paintMat mat3(frag[3].xyz, frag[4].xyz, frag[5].xyz)
#define innerCol frag[6]
#define outerCol frag[7]
#define scissorExt frag[8].xy
#define scissorScale frag[8].zw
#define extent frag[9].xy
#define radius frag[9].z
#define feather frag[9].w
#define strokeMult frag[10].x
#define strokeThr frag[10].y
#define texType int(frag[10].z)
#define shaderType int(frag[10].w)
#define hasMask int(frag[11].x)

uniform sampler2D tex;
uniform sampler2D masktex;
uniform vec2 viewSize;

varying vec2 ftcoord;
varying vec2 fpos;

float sdroundrect(vec2 pt, vec2 ext, float rad) {
    vec2 ext2 = ext - vec2(rad,rad);
    vec2 d = abs(pt) - ext2;
    return min(max(d.x,d.y),0.0) + length(max(d,0.0)) - rad;
}

// Scissoring
float scissorMask(vec2 p) {
    vec2 sc = (abs((scissorMat * vec3(p,1.0)).xy) - scissorExt);
    sc = vec2(0.5,0.5) - sc * scissorScale;
    return clamp(sc.x,0.0,1.0) * clamp(sc.y,0.0,1.0);
}

#ifdef EDGE_AA
// Stroke - from [0..1] to clipped pyramid, where the slope is 1px.
float strokeMask() {
    return min(1.0, (1.0-abs(ftcoord.x*2.0-1.0))*strokeMult) * min(1.0, ftcoord.y);
    // Using this smoothstep preduces maybe better results when combined with fringe_width of 2, but it may look blurrier
    // maybe this should be controlled via flag
    //return smoothstep(0.0, 1.0, (1.0-abs(ftcoord.x*2.0-1.0))*strokeMult) * smoothstep(0.0, 1.0, ftcoord.y);
}
#endif

void main(void) {
    vec4 result;

    float scissor = scissorMask(fpos);

#ifdef EDGE_AA
    float strokeAlpha = strokeMask();

    if (strokeAlpha < strokeThr)
        discard;
#else
    float strokeAlpha = 1.0;
#endif

    if (shaderType == 0) {
        // Gradient

        // Calculate gradient color using box gradient
        vec2 pt = (paintMat * vec3(fpos, 1.0)).xy;

        float d = clamp((sdroundrect(pt, extent, radius) + feather*0.5) / feather, 0.0, 1.0);
        vec4 color = mix(innerCol,outerCol,d);

        result = color;
    } else if (shaderType == 1) {
        // Image

        // Calculate color from texture
        vec2 pt = (paintMat * vec3(fpos, 1.0)).xy / extent;

        vec4 color = texture2D(tex, pt);

        if (texType == 1) color = vec4(color.xyz * color.w, color.w);
        if (texType == 2) color = vec4(color.x);

        // Apply color tint and alpha.
        color *= innerCol;

        result = color;
    } else if (shaderType == 2) {
        // Stencil fill
        result = vec4(1,1,1,1);
    }

    if (hasMask == 1) {
        // Textured tris
        vec4 mask = texture2D(masktex, ftcoord);
        mask = vec4(mask.x);

        //if (texType == 1) mask_color = vec4(mask_color.xyz * mask_color.w, mask_color.w);
        //if (texType == 2) mask_color = vec4(mask_color.x);

        mask *= scissor;
        result *= mask;
    } else if (shaderType != 2) { // Not stencil fill
        // Combine alpha
        result *= strokeAlpha * scissor;
    }

    gl_FragColor = result;
}
