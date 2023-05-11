/*
    Hello Triangle
    https://learnopengl.com/Getting-started/Hello-Triangle
    Exercise 3
    Create two shader programs where the second program uses a different fragment shader that outputs the color yellow;
    draw both triangles again where one outputs the color yellow
*/

extern crate glfw;
use glfw::{Action, Context, Key, Window};
use learn_opengl::glad::gl33::{self as gl, types::*};
use std::ffi::{CStr, CString};
use std::{ptr, str};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct Color<'a> {
    rgba: &'a str, // RGBA 0-1 Colors
    description: &'a str,
}

const ORANGE: Color<'static> = Color {
    rgba: "1.0f, 0.5f, 0.2f, 1.0f",
    description: "Orange",
};
const YELLOW: Color<'static> = Color {
    rgba: "1.0f, 0.8f, 0.0f, 1.0f",
    description: "Yellow",
};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
void main()
{
    FragColor = vec4([color]);
}
"#;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, _events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "Learn OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("failed to create GLFW window");

    window.make_current();

    // glad: load all OpenGL function pointers
    // ---------------------------------------
    let mut gl =
        gl::load(|symbol| glfw.get_proc_address_raw(symbol) as *const std::os::raw::c_void);

    set_viewport_size(
        &mut gl,
        // convert u32 to i32, fail if out of range
        SCR_WIDTH.try_into().unwrap(),
        SCR_HEIGHT.try_into().unwrap(),
    );

    let (orange_shader_program, yellow_shader_program, vao_arr) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertex_shader = match create_vertex_shader(&gl, VERTEX_SHADER_SOURCE) {
            Ok(shader) => shader,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        // orange fragment shader
        let orange_fragment_shader =
            match create_fragment_shader(&gl, FRAGMENT_SHADER_SOURCE, ORANGE) {
                Ok(shader) => shader,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

        // yellow fragment shader
        let yellow_fragment_shader =
            match create_fragment_shader(&gl, FRAGMENT_SHADER_SOURCE, YELLOW) {
                Ok(shader) => shader,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

        // link shaders
        let orange_shader_program = match link_shaders(&gl, vertex_shader, orange_fragment_shader) {
            Ok(shader) => shader,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        let yellow_shader_program = match link_shaders(&gl, vertex_shader, yellow_fragment_shader) {
            Ok(shader) => shader,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        // delete the shaders as they're linked into our program
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(orange_fragment_shader);
        gl.DeleteShader(yellow_fragment_shader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let left_triangle: [f32; 9] = [
            -1.0, -0.5, 0.0, // left
            0.0, -0.5, 0.0, // right
            -0.5, 0.5, 0.0, // top
        ];

        let right_triangle: [f32; 9] = [
            0.0, -0.5, 0.0, // left
            1.0, -0.5, 0.0, // right
            0.5, 0.5, 0.0, // top
        ];

        let mut vbo_arr: [u32; 2] = [0; 2];
        let mut vao_arr: [u32; 2] = [0; 2];
        gl.GenVertexArrays(2, vao_arr.as_mut_ptr());
        gl.GenBuffers(2, vbo_arr.as_mut_ptr());
        // copy left triangle vertices array into a buffer for OpenGL to use
        gl.BindVertexArray(vao_arr[0]);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo_arr[0]);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (left_triangle.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            left_triangle.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl.EnableVertexAttribArray(0);

        // copy right triangle vertices array into a buffer for OpenGL to use
        gl.BindVertexArray(vao_arr[1]);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo_arr[1]);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (right_triangle.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            right_triangle.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl.EnableVertexAttribArray(0);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl.BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // optional: de-allocate all resources once they've outlived their purpose:
        // ------------------------------------------------------------------------
        gl.DeleteBuffers(2, vbo_arr.as_ptr());

        (orange_shader_program, yellow_shader_program, vao_arr)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // input
        // -----
        process_input(&mut window);

        // render
        // ------
        unsafe {
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            // draw our left triangl
            gl.UseProgram(orange_shader_program);
            gl.BindVertexArray(vao_arr[0]);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);

            // draw our right triangle
            gl.UseProgram(yellow_shader_program);
            gl.BindVertexArray(vao_arr[1]);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        window.glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl.DeleteVertexArrays(2, vao_arr.as_ptr());
        gl.DeleteProgram(orange_shader_program);
        gl.DeleteProgram(yellow_shader_program);
    }

    // glfw: terminate, clearing all previously allocated GLFW resources.
    // ------------------------------------------------------------------
    // dropped when out of scope
}

// process all input: query GLFW whether relevant keys are pressed/released this frame and react accordingly
// ---------------------------------------------------------------------------------------------------------
fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}

// glfw: whenever the window size changed (by OS or user resize) this callback function executes
// ---------------------------------------------------------------------------------------------
fn set_viewport_size(gl: &mut gl::Gl, width: i32, height: i32) {
    unsafe { gl.Viewport(0, 0, width, height) };
}

fn create_vertex_shader(gl: &gl::Gl, source: &str) -> Result<u32, String> {
    let shader = unsafe {
        // vertex shader
        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_cstring = CString::new(source.as_bytes()).unwrap();
        let vertex_shader_ptr = &vertex_shader_cstring.as_ptr();
        gl.ShaderSource(vertex_shader, 1, vertex_shader_ptr, ptr::null());
        gl.CompileShader(vertex_shader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let len = 512;
        let mut info_log = Vec::with_capacity(len as usize - 1); // minus the null terminator        gl.GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        gl.GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetShaderInfoLog(
                vertex_shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{:?}",
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            ));
        };
        vertex_shader
    };
    Ok(shader)
}

fn create_fragment_shader(gl: &gl::Gl, source: &str, color: Color) -> Result<u32, String> {
    let source_string = String::from(source);
    let shader_string = source_string.replace("[color]", color.rgba);
    let shader = unsafe {
        // orange fragment shader
        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_cstring = CString::new(shader_string.as_bytes()).unwrap();
        let fragment_shader_ptr = &fragment_shader_cstring.as_ptr();
        gl.ShaderSource(fragment_shader, 1, fragment_shader_ptr, ptr::null());
        gl.CompileShader(fragment_shader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let len = 512;
        let mut info_log = Vec::with_capacity(len as usize - 1); // minus the null terminator
        gl.GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetShaderInfoLog(
                fragment_shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n Description: {}\n{:?}",
                color.description,
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            ));
        };
        fragment_shader
    };
    Ok(shader)
}

fn link_shaders(gl: &gl::Gl, vertex_shader: u32, fragment_shader: u32) -> Result<u32, String> {
    let shader_program = unsafe {
        let program = gl.CreateProgram();
        gl.AttachShader(program, vertex_shader);
        gl.AttachShader(program, fragment_shader);
        gl.LinkProgram(program);

        // check for linker errors
        let mut success = gl::FALSE as GLint;
        let len = 512;
        let mut info_log = Vec::with_capacity(len as usize - 1); // minus the null terminator
        gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{:?}",
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            ));
        };

        program
    };
    Ok(shader_program)
}
