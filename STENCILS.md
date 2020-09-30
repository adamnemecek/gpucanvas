# stencils

* we are debugging concave stencil
* concave fill

the default values are 
    * stencilcompare function = always
    * and operations are keep for all
    * default masks as 0xffff_ffff

# metal
* convex_fill
  * no stencil
* concave_fill
  *  match cmd.fill_rule {
                FillRule::NonZero => {
                    //gl::StencilFunc(gl::EQUAL, 0x0, 0xff),
                    encoder.set_stencil_reference_value(0xff);
                }
                FillRule::EvenOdd => {
                    // gl::StencilFunc(gl::EQUAL, 0x0, 0x1),
                    encoder.set_stencil_reference_value(0x1);
                }
            }
  * fill_shape_stencil_state

  * FillRule::NonZero => {
                //gl::StencilFunc(gl::NOTEQUAL, 0x0, 0xff),
                encoder.set_stencil_reference_value(0xff);
            }
            FillRule::EvenOdd => {
                // gl::StencilFunc(gl::NOTEQUAL, 0x0, 0x1),
                encoder.set_stencil_reference_value(0x1);
            }
  * fill_stencil_state
* stroke
  * no stencils
* stencil_stroke
  * stroke_shape_stencil_state
  * stroke_anti_alias_stencil_state
  * stroke_clear_stencil_state
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
