/*
    Hello Triangle
    https://learnopengl.com/Getting-started/Hello-Triangle
    Exercise 1
    Adjust the vertex shader so that the triangle is upside down
*/

extern crate glfw;
use glad::gl33::{self as gl, types::*};
use glfw::{Action, Context, Key, Window};
use std::ffi::{CStr, CString};
use std::{ptr, str};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

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
    // Hex #ff8033, RBGA 0-255 rgba(255, 128, 51, 1)
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f); // RGBA 0-1
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

    let (shader_program, vao) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        // vertex shader
        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_cstring = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        let vertex_shader_ptr = &vertex_shader_cstring.as_ptr();
        gl.ShaderSource(vertex_shader, 1, vertex_shader_ptr, ptr::null());
        gl.CompileShader(vertex_shader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let len = 512;
        let mut info_log = Vec::with_capacity(len as usize - 1); // minus the null terminator
        gl.GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetShaderInfoLog(
                vertex_shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            );
        };

        // fragment shader
        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_cstring = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        let fragment_shader_ptr = &fragment_shader_cstring.as_ptr();
        gl.ShaderSource(fragment_shader, 1, fragment_shader_ptr, ptr::null());
        gl.CompileShader(fragment_shader);

        // check for shader compile errors
        gl.GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetShaderInfoLog(
                fragment_shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            );
        };

        // link shaders
        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);

        // check for linker errors
        gl.GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl.GetProgramInfoLog(
                shader_program,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                CStr::from_ptr(info_log.as_ptr()).to_string_lossy(),
            );
        };

        // delete the shaders as they're linked into our program
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices: [f32; 18] = [
            // Left Triangle
            -1.0, -0.5, 0.0, // left
            0.0, -0.5, 0.0, // right
            -0.5, 0.5, 0.0, // top
            // Right Triangle
            0.0, -0.5, 0.0, // left
            1.0, -0.5, 0.0, // right
            0.5, 0.5, 0.0, // top
        ];

        let mut vbo = 0;
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        // bind the Vertex Array Object
        gl.BindVertexArray(vao);

        // copy our vertices array into a buffer for OpenGL to use
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        // set the vertex attributes pointers
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
        gl.DeleteBuffers(1, &vbo);

        (shader_program, vao)
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

            // draw our first triangle
            // use our shader program when we want to render an object
            gl.UseProgram(shader_program);
            // Core OpenGL requires that we use a VAO so it knows what to do with our vertex inputs.
            // If we fail to bind a VAO, OpenGL will most likely refuse to draw anything.
            gl.BindVertexArray(vao);
            // draw the object
            // count 6, since there are 6 vertices, two triangles
            gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        window.glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl.DeleteVertexArrays(1, &vao);
        gl.DeleteProgram(shader_program);
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
