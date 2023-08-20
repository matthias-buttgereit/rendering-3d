# TinyRenderer in Rust
This is my project of implementing the tinyrenderer described in this tutorial [https://github.com/ssloy/tinyrenderer] by ssloy in Rust.

I started out by setting single pixels in an ImageBuffer and saving that to a png file (using the image crate). Afterwards I drew single lines and when that worked I managed to draw the edges of triangles.

The tutorial provides some obj files that contain a (long) list of vertices, so points in 3D, and a list of faces, which are triangles in 3D always consisting of three of those previous vertices. Using the nom crate I parsed the file and was able to draw all the faces/triangles it contained. Only the edges at first so I tried to paint all triangles white for the start.

picture

As you can see painting all triangles plain white removed any sort of structure from the 3D model. So I calculated the normal vector of each face, implemented a light-vector (shining from the same position as the viewer) and shaded every triangle based on the dot product between the their normal vector and the light direction. That way triangle facing directly towards the viewer get painted in pure white while triangles facing away get painted darker, depending on how much the are facing away.

The tutorial also included a texture file. It is very much just an 2D image with every vertex of a triangle corresponding to a point on the texture image. So given a triangle we can map all three corners of it to three points on the image, hence giving us a new triangle on the texture image. In order to paint every pixel of one triangle of the model I just had to interpolate where that pixel was in the original triangle and what its corresponding position would be on the texture image (within the triangle on there).

picture

So far I converted the 3D model to the 2D image simply by dropping the z (depth) component. Which is not entirely true, I kept the value around in a zbuffer to not draw any triangle that behind other triangles. Still this method made for a good first draft but not a realistic perspective projection. The tutorial provided a good mathematical explanation of how to do some matrix multiplication to eventually turn every 3D point from the model into a perspective 2D point on the generated image given a position for the viewer - or the camera. With this code added it was easy to change the camera and view on the model.

picture
