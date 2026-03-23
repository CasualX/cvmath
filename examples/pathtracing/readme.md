Path tracing implementation

Following along Sebastian Lague's _Coding Adventure: Ray Tracing_

https://www.youtube.com/watch?v=Qz0KTGYJtUk

The lighting model is slightly changed to "Simplified Metallic-Roughness PBR with Emissive Support".

Using ImageMagick and oxipng to convert the output to PNG:

```bash
image=lowpoly_tree && \
convert $image.ppm -strip -gamma 1.0 $image.png && \
oxipng -omax $image.png
```
