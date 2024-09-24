extern crate opengl_graphics;
extern crate glfw_window;
extern crate piston;
extern crate graphics;

mod consts;
mod game;

use consts::{DEFAULT_WINDOW_SIZE, OPEN_GL};
use game::Game;

use glfw_window::GlfwWindow;
use piston::{EventSettings, Events, MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent, ResizeEvent, UpdateEvent, WindowSettings};


fn main() {
    let mut window: GlfwWindow = WindowSettings::new("Conway's Game Of Life", DEFAULT_WINDOW_SIZE)
        .graphics_api(OPEN_GL)
        .exit_on_esc(true)
        .automatic_close(true)
        .resizable(true)
        .build()
        .expect("Could not create window");

    let mut game = Game::new();
    let mut event_manager = Events::new(EventSettings::new());

    while let Some(event) = event_manager.next(&mut window) {
        if let Some(resize_arg) = event.resize_args() { // When new != last
            game.resize(&resize_arg);
        }
        if let Some(render_arg) = event.render_args() { // Every tick
            game.render(&render_arg);
        }
        if let Some(update_arg) = event.update_args() { // Every tick
            game.update(&update_arg);
        }
        if let Some(position) = event.mouse_cursor_args() { // When new mouse_pos != last_mouse_pos
            game.update_mouse_position(position);
        }
        if let Some(pressed_button) = event.press_args() { // When starting to press (1 tick)
            game.handle_button_press(pressed_button);
        }
        if let Some(released_button) = event.release_args() { // When stopping (1 tick)
            game.handle_button_release(released_button);
        }
    }
}