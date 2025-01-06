# Foreword

## Why another OpenGL tutorial?

Like many, I took my first steps with OpenGL around 2010 with the [legendary NeHe tutorials on nehe.gamedev.net](https://nehe.gamedev.net/). But back then they were already starting to become a bit outdated. They were Windows-only and use the `glBegin`-`glEnd`-style of OpenGl programming which is no longer supported in current versions of OpenGl since OpenGl 3.3. (Today, the NeHe tutorials are even listed as “Legacy Tutorials” on their page.)

But apart from them being slightly outdated, what actually annoyed me was the usage of a helper libraries that created thin wrappers and helper functions around the OpenGL API, like [GLU](https://en.wikipedia.org/wiki/OpenGL_Utility_Library), for example. While helper libraries can lift some amout of work, I feel they are diametrical to a basic tutorial because they sometimes hide complexity or make very simple things seem more magical than they really are and thus blur the understanding one can have about OpenGl. But one of the worst ones was GLAux, which felt like a set of functions that purely stop you from learning how things work in OpenGL. Nowadays even the NeHe tutorials [warn about using GLAux](https://nehe.gamedev.net/tutorial/creating_an_opengl_window_(win32)/13001/)

<div style="width:30%; margin: auto"><hr></div>

A few years later, around 2014, I started playing around with OpenGL again, this time on Linux and mostly without helper libraries. I found some setup code that creates an OpenGL context with the [SDL2 library](https://www.libsdl.org/). I cannot recall where it came from. But I think I got some (probably most) of the OpenGL code I used from the [OpenGl Tutorial on opengl-tutorial.org](http://www.opengl-tutorial.org/).

This tutorial still is really good and I had a lot of fun playing around and got to a point where I could just extend my project into new directions no longer bound to the tutorial.

But, alas, this was written in C++. They say “if all you have is a hammer, everything looks like a nail”. All I knew back then was OOP, and thus, everything started to look like an object. I made a complete mess in the end and at some point forgot about it.

<div style="width:30%; margin: auto"><hr></div>

Nowadays there also exists a quite good looking tutorial at the [Learn OpenGL course on learnopengl.com](https://learnopengl.com/Introduction).

If you are reading my tutorial and are missing something of the more advanced techniques of OpenGL programming, I really encourage you to check out both the [OpenGl Tutorial on opengl-tutorial.org](http://www.opengl-tutorial.org/) and the [Learn OpenGL course on learnopengl.com](https://learnopengl.com/Introduction).

<div style="width:30%; margin: auto"><hr></div>

Around 2017 I started learning Rust and really fell in love with this language.

There already exists at least one other OpenGl tutorial based on Rust: [Learn OpenGL Rust](https://rust-tutorials.github.io/learn-opengl/). The main reason that I am writing another one is that that one is based on SDL and I wanted to have a more lightweight library for window and OpenGl context creation.

What I wanted was a small library with minimal overhead but at the same time support for Wayland.

Finally, in 2024, I discovered the `winit` and `glutin` crates by the [“Rust Windowing” community](https://github.com/rust-windowing), which were offering exactly what I wanted:

- lightweight window creation,
- lightweight OpenGL context creation,
- supporting Linux, Mac and Windows,
- supporting X11, but mostly, also supporting Wayland!
- And having crate features to disable the support you don't need.

So I rolled up my sleeves, downloaded the [glutin example](https://github.com/rust-windowing/glutin/blob/master/glutin_examples/src/lib.rs) and played around with it. And the code was so simple and usable, that I thought to myself, “this needs to become a tutorial”. The only downside was, that creating the window itself is a bit of a hassle (because of inherent complexity in window creation). So I created a tiny helper crate called [`glwindow`](https://crates.io/crates/glwindow) to simplify the window creation to the most basic parameters needed for a tutorial and quite a bit beyond; Just allowing for simple window parameters as:

- size,
- resizability,
- fullscreen display,
- transparency,
- cursor capture,

and that's mostly it. You can find all the current parameters you can set in [the documentation](https://docs.rs/glwindow/latest/glwindow/struct.Window.html).

The `glwindow` crate source is currently less than 500 lines of code. If you start out using the `glwindow` crate in this tutorial, and your project grows, I encourage you to just copy its sources directly into your project and then refactor to your liking. Because `glwindow` is so tiny, this will be easy!

## What you can expect from this tutorial

This tutorial will not teach good Rust code! While I try not to write the worst Rust you will ever have seen, OpenGL is a C library. So there will be a lot of `unsafe` code.

Also I believe that OpenGL is learned better if you can read all the OpenGL code in one function (or rather two function, one for initializing and one for drawing). So the code you can read here will be very much like [spaghetti code](https://en.wikipedia.org/wiki/Spaghetti_code) (even though it's just one strand of spaghetti). But that way, you can see every OpenGL API invocation one after each other and in the order that they will be called.

But I firmly want to encourage you to change the structure of the code to your own liking as you work along this tutorial. Write your own helpers, encapsulate and generalize to your needs! That way, you can understand much better what is happening and will get much more out of the tutorial that by simple reading.

As for the `unsafe` Rust; I have a chapter planned on how to structure things and help encapsulate the unsafety into a small module or crate. If you're really keen on reading that, check back in the summer of 2025.

## How to continue afterwards

A tutorial can only be the start of an adventure!

As mentioned above, be sure to have a look at both the [OpenGl Tutorial on opengl-tutorial.org](http://www.opengl-tutorial.org/) and the [Learn OpenGL course on learnopengl.com](https://learnopengl.com/Introduction). Both tutorials should provide helpful information for readers of this tutorial when interested in certain details that are not discussed here!

You can find [links to documentation and book recommondations](books.html) in one of the final chapters of this tutorial.
