# shaders

* gpucanvas has only three states,
```rust
enum ShaderType {
    FillGradient, FillImage, FillStencil
}


impl ShaderType {
    pub fn to_f32(self) -> f32 {
        match self {
            Self::FillGradient => 0.0,
            Self::FillImage => 1.0,
            Self::Stencil => 2.0,
        }
    }
}

```

* nanovg has 4
```rust
enum NVGMTLShaderID {
    NVGMTL_SHADER_FILLGRAD,
    NVGMTL_SHADER_FILLIMG,
    NVGMTL_SHADER_SIMPLE,
    NVGMTL_SHADER_IMG
};
```