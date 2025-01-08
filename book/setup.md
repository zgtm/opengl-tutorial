# Preparations

## Build system setup

First of all, you will need to install [Rust](https://www.rust-lang.org/)! You can find download instructions for Rust for your operating system under [rustup.rs](https://rustup.rs/).

If this is your first time using Rust, don't worry. We will go through all changes step-by-step and every change you'll need to do we be written out. However you will probably profit from learning rust, for which I recommend reading [the fantastic Rust Book](https://doc.rust-lang.org/book/ch00-00-introduction.html).

Here the basic steps for installing Rust on all three major operating systems:

- [Windows](#windows-setup)
- [MacOS](#macos-setup)
- [Linux](#linux-setup)

I'm using Linux for writing this tutorial, but I've tested all examples in all three major operating systems! :)

### Windows setup

Rust can be installed by downloading [`rust-init.exe`](https://win.rustup.rs/x86_64) from [rustup.rs](https://rustup.rs/).

By default, the Rust installer will install the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) so you can compile Windows applications with Rust. This is needed to run the examples from this tutorial. You can also download and install the [Windows SDK from Microsoft directly](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) if you prefer that or did not install it when you installed Rust.

Be sure to add the Rust binarys to your path when asked (the default), so you can invoke `cargo` from the command line!

### MacOS setup

Rust can most easily be installed by running rustup. In your shell, type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

In order to compile the OpenGL examples in this tutorial, you need to install the Xcode Command Line Tools

MacOS will automatically ask you to install the Xcode Command Line Tools, if they are not installed already, the first time you are compiling any of the examples in this tutorial.

However you can also install them manually by running:
```bash
xcode-select --install
```

### Linux setup

Rust can most easily be installed by running rustup. Install `curl` if it isn't already installed. Then, in your shell, type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Alternatively you can install `cargo` and `rustc` with your package manager. Note that you need at least Rust 1.70, which should be included in most current stable distributions (except Debian, which currently only ships Rust 1.63).

You will also need to install the base development tools that go under different names in different distributions:

Debian- / Ubuntu-based:
```
sudo apt-get install build-essentials
```

Fedora- / Redhat-based:
```
sudo dnf group install c-development development-tools
```

Arch-based:
```
sudo pacman base-devel
```

SuSE-based:
```
sudo zypper install devel_basis
```
