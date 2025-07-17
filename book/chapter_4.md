# Drawing a Triangle with Colours

Next, we want to make our triangle a bit more colourful. Instead of having the whole triangle be of the same colour, we can make each vertex have a different colour. OpenGL will then interpolate the colours between the vertices and we'll have a nice gradient.

### Vertex buffer

First of all we need to add the colour information to our vertex buffer. We could in principle also create a new buffer for the colour information, but since the colour information for each vertex is needed at the same time as the position information, it makes sense to store them as close to each other as possible. This helps with [cache locality](https://en.wikipedia.org/wiki/Locality_of_reference).

So we will store our vertex data like this:

\\([x_1, y_1, z_1, \\)&nbsp; &nbsp;\\( r_1, g_1, b_1, \\)<br>
&nbsp;\\( x_2, y_2, z_2, \\)&nbsp; &nbsp;\\( r_2, g_2, b_2,\\)<br>
&nbsp;\\(…]\\)

with \\((x_1, y_1, z_1)\\) as before describing the position of the first vertex and \\((r_1, g_1, b_1)\\) describing the colour of the the first vertex as an RGB value, and so on.

This is straightforward to translate to rust:
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:21:25}}
```

### Vertex Shader
Now our vertex shader has the glorious task of taking the colour value for each vertex … and just passing it on.

We need to define an input variable for the colour,

```c
{{#include ../chapter4_coloured_triangle/src/main.rs:31}}
```

and an output variable,

```c
{{#include ../chapter4_coloured_triangle/src/main.rs:33}}
```

and then the `main` function will just set the output to the input:


```c
{{#include ../chapter4_coloured_triangle/src/main.rs:37}}
```

The complete vertex shader now looks like this:


```c
{{#include ../chapter4_coloured_triangle/src/main.rs:27:39}}
```

### Fragment shader

The fragment shader now has a similarly glorious task as the vertex shader.

First we define the input and output variables:
```c
{{#include ../chapter4_coloured_triangle/src/main.rs:44}}
```
```c
{{#include ../chapter4_coloured_triangle/src/main.rs:46}}
```

And now, for every pixel, we get the interpolated colour value from OpenGL and set the pixel colour to that exact value. But additionally we also set the alpha-value to 1.0 to have our triangle be non-transparent:

```c
{{#include ../chapter4_coloured_triangle/src/main.rs:49}}
```

The resulting complete shader:

```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:41:51}}
```

### Shader attributes
Finally, all that's left to do is to tell OpenGL about our change of the `VERTEX_DATA` array, and wire up the right values of the array to the vertex-shader's input variables `position` and `color`.

The code for the attribute of `position` (`pos_attrib`) is almost the same as before, except that we set the stride parameter in `gl.VertexAttribPointer` to 6 instead of 3, as now the vertex position information starts at exery 6th value.

For the attribute of `color` (`color_attrib`), we have almost the same code, but here we not only set the stride parameter to 6, but also the offset to 3, as the colour information first starts _after_ the first three position values:

```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:97:119}}
```

Now, we can run our code and it should look something like this:

<img src="coloured_triangle.png" style="width: 50%; margin-left: 25%;" alt="Our first coloured triangle">

A nice colourful triangle!
## Play Around With It

 - Instead of our cyan-magenta-yellow-triangle, try to make a classical RGB triangle where one vertex is just fully red, one vertex is just fully green and one vertex is just fully blue:

<img src="coloured_triangle_rgb.png" style="width: 50%; margin-left: 25%;" alt="Our first coloured triangle in RGB">

 - Since the \\(z\\)-data of all our vertexes is 0 all the time anyway, try to see if you can just drop it from the vertex buffer and instead set it to zero in the fragment shader. You will need to modify both calls to `gl.VertexAttribPointer` at five locations in the code in addition to the changed vertex buffer `VERTEX_DATA` and the changed fragment shader.

## Full code

As always, here comes the full code of everything we've done in all the chapters before and this chapter (though some things might just reference previous chapters):

### Cargo.toml

Unchanged from [Chapter 2's Cargo.toml](chapter_2.html#cargotoml).

### build.rs

Unchanged from [Chapter 2's build.rs](chapter_2.html#buildrs).

### src/main.rs
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs}}
```
