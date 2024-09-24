use glfw_window::OpenGL;

pub const OPEN_GL: OpenGL = OpenGL::V4_5;

pub const BG_COLOR: [f32; 4] = [0.08, 0.08, 0.08, 1.0];
pub const CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
// pub const CELL_SIZE: f64 = 1.0;
// /// In `WorldPosition`, length of a cell
// pub const DEFAULT_ZOOM: f64 = 10.0;
pub const ZOOM_SPEED: f64 = 1.0;
pub const SUPER_ZOOM_SPEED: f64 = 1.5;

/// In `ScreenPosition`
pub const DEFAULT_WINDOW_SIZE: [f64; 2] = [780.0, 360.0];

/// In `WorldPosition`
pub const WORLD_WIDTH: f64 = 500.0;
/// In `WorldPosition`
pub const WORLD_HEIGHT: f64 = 500.0;
/// In `WorldPosition`
/// 
/// Overriden by `WORLD_WIDTH` if the latter is smaller
pub const DEFAULT_CAMERA_WIDTH: f64 = 100.0;
// /// In `WorldPosition`
// /// 
// /// Overriden by `WORLD_HEIGHT` if the latter is smaller
// pub const DEFAULT_CAMERA_HEIGHT: f64 = DEFAULT_WINDOW_SIZE[1] * DEFAULT_CAMERA_WIDTH / DEFAULT_WINDOW_SIZE[0];

pub const DT_BEFORE_UPDATE: f64 = 1.0; // in seconds
/// Camera moving speed
pub const SPEED: f64 = 45.0;
/// Camera moving speed
pub const SUPER_SPEED: f64 = 200.0;

pub const GRID_WIDTH: usize = WORLD_WIDTH as usize;
pub const GRID_HEIGHT: usize = WORLD_HEIGHT as usize;
// pub const GRID_SHAPE: (usize, usize) = (GRID_WIDTH, GRID_HEIGHT);