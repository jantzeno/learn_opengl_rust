# Learn OpenGL with Unsafe Rust

An attempt to follow [Learn OpenGL](https://learnopengl.com) using [Usafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html), [glfw-rs](https://crates.io/crates/glfw), and [glad 2](https://gen.glad.sh/) GL loader.

This is a Cargo Workspace. Each section is a package within the workspace.

The `bin` folder of each package is organized to match the tutorials from LearnOpenGL.

Each section follows the C/C++ tutorial code and is not idiomatic Rust.

## Completed Sections (packages)

Getting Started
- Hello Window
- Hello Triangle
    - EBO Section
    - Exercises

## Prerequisites

### GLFW

Fedora

`dnf install glfw glfw-devel`

### Glad

The `glad/src/gl33.rs` file was generated with GL 3.3 Core.
The `glad/src/gl46.rs` file was generated with GL 4.6 Core.

## How to run a specific tutorial

`
cargo run -p [package_name] --bin [section_name]
`

Example:

`
cargo run -p getting_started --bin hello_window
`

## How to list targets in a package

`
cargo run -p [package_name] --bin
`

Example: 

```
$ cargo run -p getting_started --bin
error: "--bin" takes one argument.
Available binaries:
    hello_triangle
    hello_triangle_ebo
    hello_triangle_ex_1
    hello_triangle_ex_2
    hello_triangle_ex_3
    hello_window
```