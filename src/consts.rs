use glfw_window::OpenGL;

pub const OPEN_GL: OpenGL = OpenGL::V4_5;

pub const BG_COLOR: [f32; 4] = [0.08, 0.08, 0.08, 1.0];
pub const CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const CELL_SIZE: f64 = 10.0;

pub const DEFAULT_WINDOW_WIDTH: f64 = 780.0; // Should be divisible by CELL_SIZE
pub const DEFAULT_WINDOW_HEIGHT: f64 = 360.0; // Same
pub const DEFAULT_WINDOW_SIZE: [f64; 2] = [DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT];

pub const DT_BEFORE_UPDATE: f64 = 1.0; // in seconds

pub const GRID_WIDTH: usize = (DEFAULT_WINDOW_WIDTH / CELL_SIZE) as usize;
pub const GRID_HEIGHT: usize = (DEFAULT_WINDOW_HEIGHT / CELL_SIZE) as usize;
// pub const GRID_SHAPE: (usize, usize) = (GRID_WIDTH, GRID_HEIGHT);