mod grid;
mod game_state;
mod positions;
mod camera;

use grid::GameGrid;
use game_state::GameState;
use positions::*;
use camera::Camera;
use crate::consts::*;

use std::collections::HashSet;

use opengl_graphics::GlGraphics;
use piston::{Button, Key, MouseButton, RenderArgs, ResizeArgs, UpdateArgs};




#[derive(Clone, Copy)]
enum Speed {
    InsanelyFast,
    SuperFast,
    VeryFast,
    Fast,
    Normal,
    Slow,
    VerySlow,
    SuperSlow
}

impl From<Speed> for f64 {
    fn from(value: Speed) -> Self {
        match value {
            Speed::InsanelyFast => 100.0, // 100 updates per second
            Speed::SuperFast => 50.0, // 50 updates per second
            Speed::VeryFast => 20.0, // 20 updates per second
            Speed::Fast => 10.0, // 10 updates per second
            Speed::Normal => 5.0, // 5 updates per second
            Speed::Slow => 2.0, // 2 updates every second
            Speed::VerySlow => 0.5, // 2 secs for an update
            Speed::SuperSlow => 0.25 // 4 secs for an update
        }
    }
}


pub struct Game {
    gl: GlGraphics,
    camera: Camera,
    board: GameGrid,
    state: GameState,
    total_dt: f64,
    speed: Speed,
    mouse_coords: Option<[f64; 2]>,
    pressed_buttons: HashSet<Button>,
    changed_tiles: HashSet<GridPosition>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            gl: GlGraphics::new(OPEN_GL),
            camera: Camera::default(),
            board: GameGrid::new(GRID_HEIGHT, GRID_WIDTH),
            state: GameState::Pause,
            total_dt: 0.0,
            speed: Speed::Normal,
            mouse_coords: None,
            pressed_buttons: HashSet::new(),
            changed_tiles: HashSet::new(),
        }
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.camera.resize(args.window_size);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let gl = &mut self.gl;

        let context = gl.draw_begin(args.viewport());
        clear(BG_COLOR, gl);
        let transform = context.transform.trans(0.0, 0.0).rot_deg(0.0);
        let (rows, cols) = self.board.get_shape();
        for y in 0..rows {
            for x in 0..cols{
                if self.board.get(y, x).unwrap() { // Cell is alive ?
                    if let Some(screen_pos) = GridPosition(x,y).to_screen_position(&self.camera) {
                        rectangle(
                            CELL_COLOR,
                            rectangle::square(screen_pos[0], screen_pos[1], self.camera.cell_lenght()),
                            transform.clone(),
                            gl
                        );
                    }
                }
            }
        }
        
        gl.draw_end();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        
        let lshift = self.pressed_buttons.contains(&Button::Keyboard(Key::LShift));

        // Zooming and dezooming
        let zoom = self.pressed_buttons.contains(&Button::Keyboard(Key::S));
        let dezoom = self.pressed_buttons.contains(&Button::Keyboard(Key::X));
        if zoom && !dezoom {
            self.camera.zoom(args.dt, lshift);
        }
        else if !zoom && dezoom {
            self.camera.dezoom(args.dt, lshift);
        }

        // Moving the camera up, down, right or left
        let speed = if lshift {SUPER_SPEED} else {SPEED};
        let up = self.pressed_buttons.contains(&Button::Keyboard(Key::Up));
        let down = self.pressed_buttons.contains(&Button::Keyboard(Key::Down));
        let right = self.pressed_buttons.contains(&Button::Keyboard(Key::Right));
        let left = self.pressed_buttons.contains(&Button::Keyboard(Key::Left));
        if up && !down {
            self.camera.move_max([0.0, args.dt * -speed]); // Negative because y is downward
        }
        else if !up && down {
            self.camera.move_max([0.0, args.dt * speed]); // Positive for same reason
        }
        if right && !left {
            self.camera.move_max([args.dt * speed, 0.0]);
        }
        else if !right && left {
            self.camera.move_max([args.dt * -speed, 0.0]);
        }
        

        // Updating the cell grid with a new generation
        if self.state == GameState::Play {
            self.total_dt += args.dt * f64::from(self.speed);
            if self.total_dt > DT_BEFORE_UPDATE {
                self.total_dt -= DT_BEFORE_UPDATE;
                self.board.next_generation();
            }
        }
    }

    pub fn clear(&mut self) {
        self.board.clear();
    }

    fn switch_pause(&mut self) {
        self.state = !self.state;
    }

    fn increase_updates(&mut self) {
        match self.speed {
            Speed::SuperSlow => {self.speed = Speed::VerySlow},
            Speed::VerySlow => {self.speed = Speed::Slow},
            Speed::Slow => {self.speed = Speed::Normal},
            Speed::Normal => {self.speed = Speed::Fast},
            Speed::Fast => {self.speed = Speed::VeryFast},
            Speed::VeryFast => {self.speed = Speed::SuperFast},
            Speed::SuperFast => {self.speed = Speed::InsanelyFast},
            Speed::InsanelyFast => {}
        };
    }

    fn decrease_updates(&mut self) {
        match self.speed {
            Speed::InsanelyFast => {self.speed = Speed::SuperFast},
            Speed::SuperFast => {self.speed = Speed::VeryFast},
            Speed::VeryFast => {self.speed = Speed::Fast},
            Speed::Fast => {self.speed = Speed::Normal},
            Speed::Normal => {self.speed = Speed::Slow},
            Speed::Slow => {self.speed = Speed::VerySlow},
            Speed::VerySlow => {self.speed = Speed::SuperSlow},
            Speed::SuperSlow => {}
        };
    }

    /*
    fn load_glider_canon(&mut self) {
        if self.state == GameState::Pause {
            let board_size = self.board.get_shape();
            if board_size.0 > 35 && board_size.1 > 8 {
                self.clear();
                
                // Left square
                self.board.switch_state_at(4, 0).unwrap();
                self.board.switch_state_at(4, 1).unwrap();
                self.board.switch_state_at(5, 0).unwrap();
                self.board.switch_state_at(5, 1).unwrap();

                // Half-circle
                self.board.switch_state_at(2, 13).unwrap();
                self.board.switch_state_at(2, 12).unwrap();
                self.board.switch_state_at(3, 11).unwrap();
                self.board.switch_state_at(4, 10).unwrap();
                self.board.switch_state_at(5, 10).unwrap();
                self.board.switch_state_at(6, 10).unwrap();
                self.board.switch_state_at(7, 11).unwrap();
                self.board.switch_state_at(8, 12).unwrap();
                self.board.switch_state_at(8, 13).unwrap();

                // Small point
                self.board.switch_state_at(5, 14).unwrap();

                // Left ship
                self.board.switch_state_at(3, 15).unwrap();
                self.board.switch_state_at(4, 16).unwrap();
                self.board.switch_state_at(5, 16).unwrap();
                self.board.switch_state_at(5, 17).unwrap();
                self.board.switch_state_at(6, 16).unwrap();
                self.board.switch_state_at(7, 15).unwrap();

                // Right ship
                self.board.switch_state_at(4,20).unwrap();
                self.board.switch_state_at(3,20).unwrap();
                self.board.switch_state_at(2,20).unwrap();
                self.board.switch_state_at(4,21).unwrap();
                self.board.switch_state_at(3,21).unwrap();
                self.board.switch_state_at(2,21).unwrap();
                self.board.switch_state_at(1,22).unwrap();
                self.board.switch_state_at(5,22).unwrap();

                // Right bars
                self.board.switch_state_at(0, 24).unwrap();
                self.board.switch_state_at(1, 24).unwrap();
                self.board.switch_state_at(5, 24).unwrap();
                self.board.switch_state_at(6, 24).unwrap();

                // Right square
                self.board.switch_state_at(2, 34).unwrap();
                self.board.switch_state_at(2, 35).unwrap();
                self.board.switch_state_at(3, 34).unwrap();
                self.board.switch_state_at(3, 35).unwrap();
            }
        }
    } */

    pub fn update_mouse_position(&mut self, position: [f64;2]) {
        self.mouse_coords = Some(position);
        self.edit_at_position(position);
    }

    fn edit_at_position(&mut self, position: [f64;2]) {
        if !self.pressed_buttons.contains(&Button::Mouse(MouseButton::Left)) || self.state != GameState::Pause {
            return;
        }
        let grid_position = screen_to_grid(position, &self.camera);
        if !self.changed_tiles.contains(&grid_position) {
            self.board.switch_state_at(grid_position.1, grid_position.0).unwrap_or_else(
                |_| panic!("Try click ({:?},{:?})", grid_position.0, grid_position.1)
            );
            self.changed_tiles.insert(grid_position);
        }
    }

    

    pub fn handle_button_press(&mut self, button: Button) { // Only when STARTING to press
        match button {
            Button::Keyboard(key) => {
                match key {
                    Key::S => {self.pressed_buttons.insert(Button::Keyboard(Key::S));},
                    Key::X => {self.pressed_buttons.insert(Button::Keyboard(Key::X));},
                    Key::Up => {self.pressed_buttons.insert(Button::Keyboard(Key::Up));},
                    Key::Down => {self.pressed_buttons.insert(Button::Keyboard(Key::Down));}, 
                    Key::Right => {self.pressed_buttons.insert(Button::Keyboard(Key::Right));},
                    Key::Left => {self.pressed_buttons.insert(Button::Keyboard(Key::Left));},
                    Key::LShift => {self.pressed_buttons.insert(Button::Keyboard(Key::LShift));},
                    _ => {}
                }
            },
            Button::Mouse(mouse_button) => {
                match mouse_button {
                    MouseButton::Left => {
                        self.pressed_buttons.insert(Button::Mouse(MouseButton::Left));
                        if let Some(mouse_position) = self.mouse_coords {
                            self.edit_at_position(mouse_position);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    pub fn handle_button_release(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => {
                match key {
                    Key::P => self.increase_updates(),
                    Key::O => self.decrease_updates(),
                    Key::Space => self.switch_pause(),
                    Key::S => {self.pressed_buttons.remove(&Button::Keyboard(Key::S));},
                    Key::X => {self.pressed_buttons.remove(&Button::Keyboard(Key::X));},
                    Key::C => if self.state == GameState::Pause {self.clear()},
                    // Key::G => self.load_glider_canon(),
                    Key::Up => {self.pressed_buttons.remove(&Button::Keyboard(Key::Up));},
                    Key::Down => {self.pressed_buttons.remove(&Button::Keyboard(Key::Down));},
                    Key::Right => {self.pressed_buttons.remove(&Button::Keyboard(Key::Right));},
                    Key::Left => {self.pressed_buttons.remove(&Button::Keyboard(Key::Left));},
                    Key::LShift => {self.pressed_buttons.remove(&Button::Keyboard(Key::LShift));},
                    _ => {},
                }
            },
            Button::Mouse(mouse_button) => {
                match mouse_button {
                    MouseButton::Left => {
                        self.pressed_buttons.remove(&Button::Mouse(MouseButton::Left));
                        self.changed_tiles.clear();
                    },
                    _ => {}
                }
            }
            _ => {}
        } 
    }
}