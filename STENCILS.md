# stencils


# metal
* convex_fill
* convave_fill
* stroke
* stencil_stroke
* triangles
* clear_rect

## default_stencil_state
```rust

```

## fill_shape_stencil_state
```rust
front_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Always);
front_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::IncrementWrap);

back_face_stencil_descriptor.set_stencil_compare_function(metal::MTLCompareFunction::Always);
back_face_stencil_descriptor.set_depth_stencil_pass_operation(metal::MTLStencilOperation::DecrementWrap);

let stencil_descriptor = metal::DepthStencilDescriptor::new();
stencil_descriptor.set_depth_compare_function(metal::MTLCompareFunction::Always);
stencil_descriptor.set_back_face_stencil(Some(&back_face_stencil_descriptor));
stencil_descriptor.set_front_face_stencil(Some(&front_face_stencil_descriptor));
```

## fill_anti_alias_stencil_state
```rust

// glstencilfunc(equal, _, 0)
// glstencilop(keep, keep, zero)
desc.set_stencil_compare_function(metal::MTLCompareFunction::Equal);
desc.set_stencil_failure_operation(metal::MTLStencilOperation::Keep);
desc.set_depth_failure_operation(metal::MTLStencilOperation::Keep);
desc.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);
```

## fill_stencil_state
```rust
desc.set_stencil_compare_function(metal::MTLCompareFunction::NotEqual);
desc.set_stencil_failure_operation(metal::MTLStencilOperation::Zero);
desc.set_depth_failure_operation(metal::MTLStencilOperation::Zero);
desc.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);
```

# stroke_shape_stencil_state
```rust
desc.set_stencil_compare_function(metal::MTLCompareFunction::Equal);
desc.set_stencil_failure_operation(metal::MTLStencilOperation::Keep);
desc.set_depth_failure_operation(metal::MTLStencilOperation::Keep);
desc.set_depth_stencil_pass_operation(metal::MTLStencilOperation::IncrementClamp);
```

## stroke_anti_alias_stencil_state
```rust
desc.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Keep);
```

## stroke_clear_stencil_state
```rust
desc.set_stencil_compare_function(metal::MTLCompareFunction::Always);
desc.set_stencil_failure_operation(metal::MTLStencilOperation::Zero);
desc.set_depth_failure_operation(metal::MTLStencilOperation::Zero);
desc.set_depth_stencil_pass_operation(metal::MTLStencilOperation::Zero);
```


## opengl
