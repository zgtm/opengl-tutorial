# Build system setup

First of all, you will need to install Rust! You can find download instructions for Rust for your operating system under [rustup.rs](https://rustup.rs/).

If this is your first time using Rust, don't worry. We will go through all changes step-by-step and every change you'll need to do we be written out. However you will probably profit from learning rust, for which I recommend reading the fantastic [Rust Book](https://doc.rust-lang.org/book/ch00-00-introduction.html)


## Linux setup

Rust can most easily be installed by running rustup. In your shell, type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Alternatively you can install `cargo` and `rustc` with your package manager.

<!-- TODO: Add rustc version information here! -->

You will also need to install the OpenGL library development package, usually called `libopengl-dev`.

Furthermore you will need `libx11-dev`, and `libglx-dev` (for X11 support) and `libwayland-dev` (for Wayland support).

## MacOS setup

Rust can most easily be installed by running rustup. In your shell, type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Windows setup

Rust can be installed by downloading `rust-init.exe` from [rustup.rs](https://rustup.rs/). Be sure to add the Rust binarys to your path when asked, so you can invoke `cargo` from the command line!
