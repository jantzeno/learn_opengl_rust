# Learn OpenGL with Unsafe Rust

An attempt to follow [Learn OpenGL](https://learnopengl.com) using [Usafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html), [glfw-rs](https://crates.io/crates/glfw), and [glad 2](https://gen.glad.sh/) GL loader.

The `bin` folder is organized to match the sections of LearnOpenGL.

Each section follows the C/C++ tutorial code and is not idiomatic Rust.

## Prerequisites

### GLFW

Fedora

`dnf install glfw glfw-devel`

### Glad

The `glad/gl.rs` file was generated with GL 4.6 Core.

## How to run a section

`
cargo run --bin [section_name]
`

Example:

`
cargo run --bin hello_window
`