# notes

In metal nvg there are two types of functions.
First type start with the name "render"
and are set is the one set as a function pointer on the nvgcontext.

```c++
params.renderCreate = mtlnvg__renderCreate;
params.renderCreateTexture = mtlnvg__renderCreateTexture;
params.renderDeleteTexture = mtlnvg__renderDeleteTexture;
params.renderUpdateTexture = mtlnvg__renderUpdateTexture;
params.renderGetTextureSize = mtlnvg__renderGetTextureSize;
params.renderViewport = mtlnvg__renderViewport;
params.renderCancel = mtlnvg__renderCancel;
params.renderFlush = mtlnvg__renderFlush;
params.renderFill = mtlnvg__renderFill;
params.renderStroke = mtlnvg__renderStroke;
params.renderTriangles = mtlnvg__renderTriangles;
params.renderDelete = mtlnvg__renderDelete;
params.userPtr = (__bridge_retained void*)mtl;
params.edgeAntiAlias = flags & NVG_ANTIALIAS ? 1 : 0;
```

The second type are called `fill`, `convexFill`, `stroke` & `triangles`.
These are used in the `renderFlush` function. They set the state on the encoder.
Each function takes a `MNVGcall` argument.

I guess gpucanvas only requires you do implement the second type.

```c++
if (call->type == MNVG_FILL)
    [self fill:call];
else if (call->type == MNVG_CONVEXFILL)
    [self convexFill:call];
else if (call->type == MNVG_STROKE)
    [self stroke:call];
else if (call->type == MNVG_TRIANGLES)
    [self triangles:call];
```

```rust
// Create indices for a triangle fan. (This is OK because the clipped quad should always be
// convex.)
let mut indices: Vec<u32> = vec![];
for index in 1..(quad_positions.len() as u32 - 1) {
    indices.extend_from_slice(&[0, index as u32, index + 1]);
}
```

# triangle fan
* based on pathfinder
```rust
// https://www.gamedev.net/forums/topic/643945-how-to-generate-a-triangle-fan-index-list-for-a-circle-shape/

fn triangle_fan_indices(len: usize) -> Vec<u32> {
	let mut indices: Vec<u32> = vec![];
	for index in 1..(len as u32 - 1) {
		indices.extend_from_slice(&[0, index as u32, index + 1]);
	}

	indices
}

fn main() {
	let indices = triangle_fan_indices(10);
	println!("{:?}", indices);
}
```

# TODO
* [ ] investigate setViewport vs uniforms
* [ ] indexed rendering
* [ ] what is composite operation?
* [ ] vert_arr vs vert_buf

# convex_fill



| gpucanvas     | metal  |
| render         | renderflush |
                | renderGetTextureSizeForImage

* concave_fill
    * this is called fill in metalnvg

    * fundamentally, there are these steps
        * set stencil
        * uniforms (missing in mnvg)
        * draw shapes
            *
        * restore states
        * draw anti-aliased fragments
        * draw fill
* stroking
    * gpucanvas, unlike metalnvg, separates stroke and stencil_stroke into two functions
    * stencil stroke
        * fill the stroke and base without overlap
            * set uniform
            * setdepthstencilstate
            * setrenderpipelinestate
            * draw trianglestrip
        * drawn antialiased fragments
            * setuniform
            * setdepthstencilstate
            * draw trianglestrip
        * clear stencil buffer
            * setdepthstencilstate
            * setrenderpipelinestate
            * draw trianglestrip
            * setdepthstencilstate default