/*
    Shaders - Our own shader class
    https://learnopengl.com/Getting-started/Shaders
*/

extern crate glfw;
use getting_started::shader::Shader;
use getting_started::shader::ShaderBuilder;
use glad::gl33::{self as gl, types::*};
use glfw::{Action, Context, Key};
use std::{ffi::CStr, ptr};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct Window {
    source: glfw::Window,
    gl: gl::Gl,
    _glfw: glfw::Glfw,
}

fn main() {
    // glfw: initialize and configure
    // ------------------------------

    let mut window = create_window();

    set_viewport_size(
        &mut window,
        // convert u32 to i32, fail if out of range
        SCR_WIDTH.try_into().unwrap(),
        SCR_HEIGHT.try_into().unwrap(),
    );

    let (our_shader, vao) = unsafe {
        let shader = ShaderBuilder::default().build(
            &mut window.gl,
            "getting_started/src/bin/shaders_class/shader.vs",
            "getting_started/src/bin/shaders_class/shader.fs",
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices: [f32; 18] = [
            // positions     // colors
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right, red
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left, green
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top, blue
        ];

        let mut vbo = 0;
        let mut vao = 0;
        window.gl.GenVertexArrays(1, &mut vao);
        window.gl.GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        // bind the Vertex Array Object
        window.gl.BindVertexArray(vao);

        // copy our vertices array into a buffer for OpenGL to use
        window.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        window.gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        // position attribute
        window.gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        window.gl.EnableVertexAttribArray(0);

        // color attribute
        window.gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<GLfloat>() as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        window.gl.EnableVertexAttribArray(1);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        // window.gl.BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        // window.gl.BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // window.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // optional: de-allocate all resources once they've outlived their purpose:
        // ------------------------------------------------------------------------
        // window.gl.DeleteBuffers(1, &vbo);

        (shader, vao)
    };

    // render loop
    // -----------
    while !window.source.should_close() {
        // input
        // -----
        process_input(&mut window);

        // render
        draw(&mut window, vao, &our_shader);

        // glfw: terminate, clearing all previously allocated GLFW resources.
        // ------------------------------------------------------------------
        // dropped when out of scope
    }
}

// create glfw window and gl context
// --------------------------------
fn create_window() -> Window {
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
    let gl = gl::load(|symbol| glfw.get_proc_address_raw(symbol) as *const std::os::raw::c_void);

    Window {
        source: window,
        gl,
        _glfw: glfw,
    }
}

// render
// ------
fn draw(window: &mut Window, vao: u32, shader: &Shader) {
    window.source.make_current();
    unsafe {
        // clear the color buffer
        window.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        window.gl.Clear(gl::COLOR_BUFFER_BIT);

        // activate the shader
        shader.use_program(&mut window.gl);

        // Exercise 2
        // add offset to x position
        shader.set_float(
            &mut window.gl,
            CStr::from_bytes_with_nul(b"xOffset\0").unwrap(),
            0.5,
        );

        // render the triangle
        window.gl.BindVertexArray(vao);
        window.gl.DrawArrays(gl::TRIANGLES, 0, 3);
    }

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    window.source.swap_buffers();
    window.source.glfw.poll_events();
}

// process all input: query GLFW whether relevant keys are pressed/released this frame and react accordingly
// ---------------------------------------------------------------------------------------------------------
fn process_input(window: &mut Window) {
    if window.source.get_key(Key::Escape) == Action::Press {
        window.source.set_should_close(true);
    }
}

// glfw: whenever the window size changed (by OS or user resize) this callback function executes
// ---------------------------------------------------------------------------------------------
fn set_viewport_size(window: &mut Window, width: i32, height: i32) {
    unsafe { window.gl.Viewport(0, 0, width, height) };
}
