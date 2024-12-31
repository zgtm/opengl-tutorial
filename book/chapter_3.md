# Drawing a Simple Triangle

In this chapter will will draw our first triangle onto the screen!

### Extending the Render State

Our Renderer struct will need to know about three new properties:

- An OpenGL **program**: This is the set of shaders that we will want to execute on our hardware.

  The first one, the **vertex shader**, will calculate the on-screen coordinates of our triangle.
  Thus it will output a coordinate for each point of our triangle that we put in.

  The second one, the **fragment shader** (also known as **pixel shader**), will calculate the color of each pixel of the resulting on-screen triangle.
  Since we are starting with a simple single-color triangle, our fragment shader will just output a constant colour.

- An OpenGL **Vertex Array Object (VAO)**:
  This will store informations on all vertex buffers we are using and their meaning.

  In our case this will contain only our single VBO, together with the information that our vertex buffer contains
  3 sets of data (one for each point of the triangle) that each will be bound to the `position` variable of our OpenGL program and is consisting of three floats each.

- And finally an OpenGL **Vertex Buffer Object (VBO)**:
  This will just store the positions of our vertices – i.e. the three points of our triangle.

Each of these three properties will just be an integer (`GLUint`) that is meant for OpenGL to identify the corresponding internal object:

```rust
{{#include ../chapter3_triangle/src/main.rs:14:19}}
```

### Our Vertex Buffer

This will be very easy, we just define a simple static array of all our vertex coordinates for our triangle.

This will just be a single array consisting of \\([x_1, y_1, z_1, x_2, y_2, z_2, …]\\). So \\((x_1, y_1, z_1)\\) will describe the \\(x\\)-, \\(y\\)- and \\(z\\)-coordinate of our first point, respectively, \\((x_2, y_2, z_2)\\) will describe the \\(x\\)-, \\(y\\)- and \\(z\\)-coordinate of our second point, respectively, and so on. And since we want to describe a triangle, _“and so on”_ only means _one more point_.

```rust
{{#include ../chapter3_triangle/src/main.rs:21:26}}
```

[Add graphic of coordinate system here]

### Here Come the Shaders

This one of the cornerstones of our OpenGL tutorial: The shaders!

As mentioned above, we will need two shaders:

- A **vertex shader**. The vertex shader will calculate the on-screen coordinates of our triangle.
  It get a vertex coordinate from our vertex buffer as an input (which we will later tell OpenGL we want to name `position`) and outputs the on-srceen coordinates in by setting the output `gl_Position`.
  We already set the vertex buffer to contain the on-screen coordinates, so we will just output the same coordinate as as 4-dimensional vector. For now you can just ignore the fourth coordinate. Just remember that it always needs to be set to 1.0, for everything to look normal. (You can play around with a few values close to 1.0 and will see that it acts like an inverse scale factor. That is actually exactly what it does, but we will come to the reasoning for that later on!)

- A **fragment shader**. The fragment shader will calculate the color of each pixel of the resulting on-screen triangle.
  Since we are starting with a simple single-color triangle, our fragment shader will just output a constant colour.
  The color is given as a 4-dimensional vector describing a point in the RGBA-space. So the first coordinate sets a _red_-value between 0.0 and 1.0, the second _green_, the third _blue_, and finally the fourth value gives an _alpha_-value, also called “opacity”–this describes how non-transparent the colour is on a scale from 0.0 (completeley transparent) to 1.0 (completely opaque).

I choose a nice orange, here, with full red, half green, one fifth of blue and full opacity \\((r=1.0, g=0.5, b=0.2, a=1.0)\\).
Thus our shaders look like this:

```rust
{{#include ../chapter3_triangle/src/main.rs:29:43}}
```

For this we need to import `CStr` from `std::ffi` (or use `std::ffi::CStr` directly):
```rust
use std::ffi::CStr;
```

### New code in AppRenderer::new

```rust
{{#include ../chapter3_triangle/src/main.rs:48:102}}
```

### New code in AppRenderer::draw

```rust
{{#include ../chapter3_triangle/src/main.rs:104:115}}
```

### Implementation of Drop for AppRenderer

```rust
{{#include ../chapter3_triangle/src/main.rs:124:132}}
```

## Full code

### Cargo.toml

Unchanged from [Chapter 2's Cargo.toml](chapter_2.html#cargotoml).

### build.rs

Unchanged from [Chapter 2's build.rs](chapter_2.html#buildrs).

### src/main.rs
```rust
{{#include ../chapter3_triangle/src/main.rs}}
```
