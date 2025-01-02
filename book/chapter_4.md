# Drawing a Triangle with Colours

### Vertex buffer
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:21:25}}
```

### Vertex Shader
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:27:37}}
```

### Fragment shader
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:39:48}}
```


### Shader attributes
```rust
{{#include ../chapter4_coloured_triangle/src/main.rs:93:113}}
```


## Play Around With It

Since the \\(z\\)-data of all our vertexes is 0 all the time anyway, you can try to see if you can just drop it from the vertex buffer and instead set it to zero in the fragment shader. You will need to modify both calls to `gl.VertexAttribPointer` at five locations in the code in addition to the changed vertex buffer `VERTEX_DATA` and the changed fragment shader.

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
