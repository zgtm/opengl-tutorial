# Drawing a Simple Triangle

In this chapter will will draw our first triangle onto the screen!

### Extending the Render State

Our Renderer struct will need to know about three new properties:

- An OpenGL **program**: This is the set of shaders that we will want to execute on our hardware.

  The first one, the **vertex shader**, will calculate the on screen coordinates of our triangle.
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

### Definition of Vertex Buffers

```rust
{{#include ../chapter3_triangle/src/main.rs:21:26}}
```

### Definition of shaders

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
