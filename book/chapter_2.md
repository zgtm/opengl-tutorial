# Adding OpenGL Bindings and Drawing a Background Colour

In this chapter we will add the OpenGl bindings to our project, use our OpenGL context and draw a background colour on our window using OpenGL.

## OpenGL Bindings

First of all, we will need to generate the OpenGL bindings so we can use them in Rust.

We will add a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) to our project so that we can generate the OpenGL bindings when building our project. We will tell Cargo to cache the bindings and only regenerate them, if the build script changes.

We need the package `gl_generator` as a build dependency. Add the following two lines to your `Cargo.toml`:

```rust
{{#include ../chapter2_background/Cargo.toml:8:9}}
```

Now, add a new file called `build.rs` and add the following contents:

```rust
{{#include ../chapter2_background/build.rs}}
```

> **Note:** If you're interested in what the output of this buildscript is, just run `cargo build`. Afterwards, you will find a file called `target/debug/build/background-????????????????/out/gl_bindings.rs` (where the questionmarks are a hexadecimal string, e. g. in my case `cf930aed0e6f0fe0`).

Add the following lines to your `main.rs` to make the OpenGL bindings available to your code:

```rust
{{#include ../chapter2_background/src/main.rs:8:12}}
```


## OpenGL Renderer

Now, let's initialise our OpenGL renderer.

In `main.rs`, our struct `Renderer` needs to store a `Gl` handle:

```rust
{{#include ../chapter2_background/src/main.rs:15:17}}
```

Now we can implement the functions `new`, `draw` and add new function `resize` for the trait implementation of `glwindow::AppRenderer` for `Renderer`.

This will need a new import:
```rust
{{#include ../chapter2_background/src/main.rs:2}}
```

The `new` function will call `gl::Gl::load_with` and store the result in `Renderer::gl`:

```rust
{{#include ../chapter2_background/src/main.rs:21:28}}
```

The `draw` function will just render the background for our app. We do that by setting the background colour as RGBA quadruple (0.1, 0.1, 0.1, 0.9), which is a dark gray with slight transparency. After setting the background colour using `ClearColor`, we need to actually draw the background using `Clear`:

```rust
{{#include ../chapter2_background/src/main.rs:30:35}}
```

And finally the `resize` function will just tell OpenGL about the new size of our OpenGL context:

```rust
{{#include ../chapter2_background/src/main.rs:37:41}}
```


## See It in Action

This is all we need for now. Try it out! Run

```bash
cargo run
```

and lo and behold

<img src="background.png" style="width: 50%; margin-left: 25%;" alt="Our first window with background">

our window is now drawing a slightly transparent dark gray background on our window&mdash;using OpenGL!

And it even redraws when we move the window!

Now we have everything prepared to draw our first triangle in the [next chapter](chapter_3.md).

## Play Around With It

Definitely try two things:

- Change the colour of the background.
- Change the amount of transparency.

## The Full Code for This Chapter

As always, here you can find the complete code for this chapter. You can use this to get up too speed quickly!

### Cargo.toml
```toml
{{#include ../chapter2_background/Cargo.toml}}
```

### build.rs
```rust
{{#include ../chapter2_background/build.rs}}
```

### src/main.rs
```rust
{{#include ../chapter2_background/src/main.rs}}
```
