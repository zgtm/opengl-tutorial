# Drawing a Rotating Triangle

Now let's get moving – literally!

We are going to animate our triangle, by making it rotate. For this, we will just take the time since the program has started running, a give the triangle a rotation value based on that time:

\\(\varphi(t) = \frac{1}{T} \cdot t\\)

where T can be any time we would like the rotation to take. Here, we will let the triangle fully rotate once every 5 seconds, so \\(T = 5\texttt{s}\\).

### Storing in an instant

First of all, in order to measure the time since program start, we'll need to import `Instant` from `std::time`:
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:3:3}}
```

We will store the `Instant` of the start of the program in the `State` as `begin`.
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:14:16}}
```

### Uniform variables

We will add a new variable to our fragment shader, which will contain the rotation matrix to rotate our triangle. For this we will create a _uniform_ variable – "uniform" meaning that it is constant for all invocations of the shader for the same object. So in our case, we will give the same rotation matrix to the invocation of the vertex shader for all three vertices.

In order to have a reference to OpenGL concept of the uniform variable, we will add it to our `Renderer` object:

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:18:24}}
```

Inside the vertex shader, we can now define the uniform variable by
```c
{{#include ../chapter5_rotating_triangle/src/main.rs:35}}
```

Now we get the vertex position by multiplying the rotation matrix to the original vertex position from the left:
```c
{{#include ../chapter5_rotating_triangle/src/main.rs:43}}
```

So our vertex shader now looks like this:
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:32:46}}
```

Finally, in the `new` method of our `Renderer` we need to get the OpenGL reference to the uniform variable

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:128:129}}
```

and store it in the new `Renderer`:

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:131:131}}
```

### Building the rotation matrix

Firstly, we need to calculate our rotation angle \\(\varphi\\). We could simply do this:
```rust
        let time = Instant::now().duration_since(state.begin).as_millis();
        let phi = (time as f32) / 5000.0 * 2.0 * std::f32::consts::PI;
```
However, an `f32` has a precision of 23 and since \\(2^23 = 8388608\\), our time variable converted into `f32` will lose precision after \\(8388608~\textrm{ms} \approx 2.3 ~\textrm{hours}\\).

Thus it makes more sense to take the time modulo 5000:
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:136}}
```

Having our value of phi, we can now define our rotation matrix

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:139:141}}
```



With that our `draw function looks like this:
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:135:141}}
```

Inside the `unsafe` block, we can now set the uniform variable `rotation` to our rotation matrix `rotation`.

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:145:145}}
```

The two parameters

### Define the beginning of time

All that's missing now is just to initialize our app state with the current time at programm start:

```rust
{{#include ../chapter5_rotating_triangle/src/main.rs:190:192}}
```

Running the code should then give you the rotating triangle, you've been waiting for:

<video style="width: 50%; margin-left: 25%;" autoplay loop muted><source type="video/mp4" src="rotating_triangle.mp4"    /></video>

## Play Around With It

 - Let the triangle rotate around another axis: The x-axis or the z-axis!

 - Can you make the triangle rotate around another axis? (E.g. around the axis defined by the vector \\((1, 1, 0)\\)?)

 - What happens if you move the triangle by 0.6 in the z-direction (either way)? What if you move it by 0.8, 1.0 or 1.2?

## Full code

As always, here comes the full code of everything we've done in all the chapters before and this chapter (though some things might just reference previous chapters):

### Cargo.toml

Unchanged from [Chapter 2's Cargo.toml](chapter_2.html#cargotoml).

### build.rs

Unchanged from [Chapter 2's build.rs](chapter_2.html#buildrs).

### src/main.rs
```rust
{{#include ../chapter5_rotating_triangle/src/main.rs}}
```
