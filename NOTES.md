# TODO
* [ ] investigate setViewport vs uniforms
* [ ] indexed rendering
* [ ] what is composite operation?
* [ ] vert_arr vs vert_buf


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