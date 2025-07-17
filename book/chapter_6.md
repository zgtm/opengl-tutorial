# Drawing a 3D-Looking Rotating Triangle

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:18:25}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:33:53}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:137:138}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:141:141}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:153:164}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d/src/main.rs:168:169}}
```


## Using a Math Library

Now that we understand what we're doing, we can stop doing it and use a library! :)


We will use the crate [`glam`](https://crates.io/crates/glam). `glam` is a pure-rust 3D math library that offers to generate rotation and perspetive matrices for us. It can do a lot more [with matrices](https://docs.rs/glam/0.30.4/glam/f32/struct.Mat3.html) and also [with quaternions](https://docs.rs/glam/0.30.4/glam/f32/struct.Quat.html), should you wish to use them later for managing rotations.

We need to add `glam` to the `[dependencies]` section in our `Cargo.toml`:

```toml
{{#include ../chapter6_rotating_triangle_3d_glam/Cargo.toml:6}}
```

Not only does `glam` help us calculate a perspective transform, it will also take the aspect ratio of our window into account, so our triangle doesn't get all squishy. For this we need to add information about the viewport, `viewport_size: (i32, i32)`, to our `Renderer`:

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:18:26}}
```

We will initialize the viewport with `(0, 0)`, as at the point of building the renderer, the window has not been created yet, so we don't actually know what size the window has. Soon, we will add the code that sets `viewport_size` to the correct value as soon as the window has been created

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:141}}
```

Now, assuming that `viewport_size` is correctly set, we can calculate the aspect ratio of our window. With that we can calculate our rotation and perspective matrices using the `glam` methods `glam::Mat4::rotation_y` and `glam::Mat4::perspective_rh_gl`, respectively.

The `rh` in `glam::Mat4::perspective_rh_gl` means that we are generating a perspective transform for a right-handed coordinate system (as OpenGL does) and `gl` means that the near and far planes are mapped into \\(z \in [-1,1]\\), as OpenGL expects. (DirectX, in contrast, expects a left-handed coordinate system and \\(z \in [0,1]\\).)

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:149:151}}
```

Now, since we are using `glam` matrices instead of our own hard-coded arrays, we'll need to change a bit how we pass the pointer to the internal data to OpenGL. Luckily for us, `glam` matrices internally are still just a continuous strip of `f32` values in memory, we can just convert the matrices into the pointers we need:

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:155:156}}
```

In our `resize` method, we can finally set the `viewport_size` of the `Renderer`. This function will be called at least once, when the window is created. So we can be sure, that our `viewport_size` always contains the actual size of our window.

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:168}}
```



## Full code

As always, here comes the full code of everything we've done in all the chapters before and this chapter (though some things might just reference previous chapters):

### Cargo.toml
```toml
{{#include ../chapter6_rotating_triangle_3d_glam/Cargo.toml}}
```

### build.rs

Unchanged from [Chapter 2's build.rs](chapter_2.html#buildrs).

### src/main.rs
```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs}}
```
