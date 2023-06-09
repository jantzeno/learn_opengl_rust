/*
    Textures - Triangle
    https://learnopengl.com/Getting-started/Textures
*/

extern crate glfw;
extern crate image;

use common::shader::ShaderBuilder;
use glad::gl33::{self as gl, types::*};
use glfw::{Action, Context, Key};
use image::GenericImageView;
use std::ptr;

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

    // establish an owner for these objects
    let mut window = Window {
        source: window,
        gl,
        _glfw: glfw,
    };

    let (our_shader, vbo, vao, texture) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        let shader = ShaderBuilder::default().build(
            &mut window.gl,
            "getting_started/src/bin/textures_triangle/texture.vs",
            "getting_started/src/bin/textures_triangle/texture.fs",
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices: [f32; 24] = [
            // positions     // colors      // texture coords
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // bottom right, red
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom left, green
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.5, 1.0, // top, blue
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
        let stride = (8 * std::mem::size_of::<GLfloat>()) as GLsizei;
        // position attribute
        window
            .gl
            .VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        window.gl.EnableVertexAttribArray(0);

        // color attribute
        window.gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        window.gl.EnableVertexAttribArray(1);

        // texture attribute
        window.gl.VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        window.gl.EnableVertexAttribArray(2);

        // load and create a texture
        // -------------------------
        let mut texture = 0;
        window.gl.GenTextures(1, &mut texture);
        window.gl.BindTexture(gl::TEXTURE_2D, texture);
        // set the texture wrapping parameters
        window
            .gl
            .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        window
            .gl
            .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // set texture filtering parameters
        window
            .gl
            .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        window
            .gl
            .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        // load image, create texture and generate mipmaps
        let img =
            image::open("getting_started/resources/wall.jpg").expect("failed to load texture");
        let (width, height) = img.dimensions();
        let data = img.into_bytes();

        window.gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width as GLsizei,
            height as GLsizei,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        window.gl.GenerateMipmap(gl::TEXTURE_2D);
        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        // gl.BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        // gl.BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // optional: de-allocate all resources once they've outlived their purpose:
        // ------------------------------------------------------------------------

        (shader, vbo, vao, texture)
    };

    // render loop
    // -----------
    while !window.source.should_close() {
        // input
        // -----
        process_input(&mut window);

        // render
        // ------
        unsafe {
            window.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            window.gl.Clear(gl::COLOR_BUFFER_BIT);

            // bind Texture
            window.gl.BindTexture(gl::TEXTURE_2D, texture);

            // render triangle
            our_shader.use_program(&mut window.gl);
            window.gl.BindVertexArray(vao);
            window.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.source.swap_buffers();
        window.source.glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        window.gl.DeleteVertexArrays(1, &vao);
        window.gl.DeleteBuffers(1, &vbo);
        window.gl.DeleteProgram(our_shader.id);
    }

    // glfw: terminate, clearing all previously allocated GLFW resources.
    // ------------------------------------------------------------------
    // dropped when out of scope
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
fn set_viewport_size(gl: &mut gl::Gl, width: i32, height: i32) {
    unsafe { gl.Viewport(0, 0, width, height) };
}
