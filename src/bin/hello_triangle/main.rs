extern crate glfw;
use glfw::{Action, Context, Key};
use learn_opengl::glad::gl33;
use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

struct Window {
    source: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    gl: learn_opengl::glad::gl33::Gl,
}

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

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
    let gl = gl33::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    let mut window = Window {
        source: window,
        events: _events,
        gl,
    };

    set_viewport_size(
        &mut window,
        // convert u32 to i32, fail if out of range
        SCR_WIDTH.try_into().unwrap(),
        SCR_HEIGHT.try_into().unwrap(),
    );

    // render loop
    // -----------
    while !window.source.should_close() {
        // input events
        // ------------
        process_input(&mut window);

        // render
        // ------
        unsafe {
            window.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            window.gl.Clear(gl33::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.source.swap_buffers();
        window.source.glfw.poll_events();
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
fn set_viewport_size(window: &mut Window, width: i32, height: i32) {
    unsafe { window.gl.Viewport(0, 0, width, height) };
}
