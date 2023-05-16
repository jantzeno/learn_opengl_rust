pub mod shader;
pub mod shader_builder;

// shorten use statement in main.rs
// from
// use getting_started::shader::shader::Shader;
// to
// use getting_started::shader::Shader;
pub use shader::*;
pub use shader_builder::*;
