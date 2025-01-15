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


```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:18:26}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:142:142}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:151:152}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:156:158}}
```

```rust
{{#include ../chapter6_rotating_triangle_3d_glam/src/main.rs:169:169}}
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
