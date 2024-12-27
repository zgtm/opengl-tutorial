# Drawing a Simple Triangle


### New fields in AppRenderer

```rust
{{#include ../chapter3_triangle/src/main.rs:14:19}}
```

### Definition of Vertex Buffers

```rust
{{#include ../chapter3_triangle/src/main.rs:21:26}}
```

### Definition of shaders

```rust
{{#include ../chapter3_triangle/src/main.rs:28:42}}
```

For this we need to import `CStr` from `std::ffi` (or use `std::ffi::CStr` directly):
```rust
use std::ffi::CStr;
```

### New code in AppRenderer::new

```rust
{{#include ../chapter3_triangle/src/main.rs:47:101}}
```

### New code in AppRenderer::draw

```rust
{{#include ../chapter3_triangle/src/main.rs:103:114}}
```

### Implementation of Drop for AppRenderer

```rust
{{#include ../chapter3_triangle/src/main.rs:123:131}}
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
