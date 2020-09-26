# stencils


# metal
## fill_shape_stencil_state

## fill_anti_alias_stencil_state
```rust
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