/*
    Textures - Exercise 1
    https://learnopengl.com/Getting-started/Textures
*/

extern crate glfw;
extern crate image;

use getting_started::shader::ShaderBuilder;
use glad::gl33::{self as gl, types::*};
use glfw::{Action, Context, Key};
use image::GenericImageView;
use std::{ffi::CStr, mem, ptr};

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

    let (our_shader, vbo, vao, ebo, texture1, texture2) = unsafe {
        // build and compile our shader program
        // ------------------------------------

        let shader = ShaderBuilder::default().build(
            &mut window.gl,
            "getting_started/src/bin/textures_ex_1/texture.vs",
            "getting_started/src/bin/textures_ex_1/texture.fs",
        );
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let positions: [f32; 12] = [
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5, 0.5, 0.0, // top left
        ];

        let colors: [f32; 12] = [
            1.0, 0.0, 0.0, // red
            0.0, 1.0, 0.0, // green
            0.0, 0.0, 1.0, // blue
            1.0, 1.0, 0.0, // yellow
        ];

        let texture_coords: [f32; 8] = [
            1.0, 1.0, // top right
            1.0, 0.0, // bottom right
            0.0, 0.0, // bottom left
            0.0, 1.0, // top left
        ];

        // combine arrays into vertex data
        let mut vertices = gen_vertices(positions.to_vec(), colors.to_vec(), 3, 3);
        vertices = gen_vertices(vertices, texture_coords.to_vec(), 6, 2);
        // convert vec to array
        let vertices: [f32; 32] = vertices.try_into().unwrap();

        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        let mut vbo = 0;
        let mut ebo = 0;
        let mut vao = 0;
        window.gl.GenVertexArrays(1, &mut vao);
        window.gl.GenBuffers(1, &mut vbo);
        window.gl.GenBuffers(1, &mut ebo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        // bind the Vertex Array Object
        window.gl.BindVertexArray(vao);

        // copy our vertices array into a buffer for Openwindow.gl to use
        window.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        window.gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        window.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        window.gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        let stride = (8 * mem::size_of::<GLfloat>()) as GLsizei;
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
            (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        window.gl.EnableVertexAttribArray(1);

        // texture attribute
        window.gl.VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        window.gl.EnableVertexAttribArray(2);

        // load and create a texture
        // -------------------------
        // texture 1
        // ---------
        let mut texture1 = 0;
        window.gl.GenTextures(1, &mut texture1);
        window.gl.BindTexture(gl::TEXTURE_2D, texture1);
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
            image::open("getting_started/resources/container.jpg").expect("failed to load texture");
        let (width, height) = img.dimensions();
        let data = img.into_bytes();

        window.gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width as GLsizei,
            height as GLsizei,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        window.gl.GenerateMipmap(gl::TEXTURE_2D);

        // texture 2
        // ---------
        let mut texture2 = 0;
        window.gl.GenTextures(1, &mut texture2);
        window.gl.BindTexture(gl::TEXTURE_2D, texture2);
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

        let img = image::open("getting_started/resources/awesomeface.png")
            .expect("failed to load texture");
        let (width, height) = img.dimensions();
        let data = img.into_bytes();

        window.gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width as GLsizei,
            height as GLsizei,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        window.gl.GenerateMipmap(gl::TEXTURE_2D);

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        // don't forget to activate/use the shader before setting uniforms!
        shader.use_program(&mut window.gl);
        // either set it manually like so:
        window.gl.Uniform1i(
            window.gl.GetUniformLocation(
                shader.id,
                CStr::from_bytes_with_nul(b"texture1\0").unwrap().as_ptr(),
            ),
            0,
        );
        // or set it via the shader class
        shader.set_int(
            &mut window.gl,
            CStr::from_bytes_with_nul(b"texture2\0").unwrap(),
            1,
        );

        // uncomment this call to draw in wireframe polygons.
        // window.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader, vbo, vao, ebo, texture1, texture2)
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
            window.gl.ActiveTexture(gl::TEXTURE0);
            window.gl.BindTexture(gl::TEXTURE_2D, texture1);
            window.gl.ActiveTexture(gl::TEXTURE1);
            window.gl.BindTexture(gl::TEXTURE_2D, texture2);

            // render the container
            our_shader.use_program(&mut window.gl);
            window.gl.BindVertexArray(vao);
            window
                .gl
                .DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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
        window.gl.DeleteBuffers(1, &ebo);
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

fn gen_vertices(lhs: Vec<f32>, rhs: Vec<f32>, lhs_step: usize, rhs_step: usize) -> Vec<f32> {
    let mut vertices: Vec<f32> = vec![];
    for i in 0..lhs.len() / lhs_step {
        vertices.extend_from_slice(&lhs[i * lhs_step..(i + 1) * lhs_step]);
        vertices.extend_from_slice(&rhs[i * rhs_step..(i + 1) * rhs_step]);
    }
    vertices
}
