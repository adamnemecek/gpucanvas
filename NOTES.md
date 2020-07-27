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


These functions only.

The second type are called `fill`, `convexFill`, `stroke`, `triangles`, `stroke`.
These are used in the `renderFlush` function and only really set the state on the encoder.

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