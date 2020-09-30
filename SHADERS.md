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