use glfw_window::OpenGL;

pub const OPEN_GL: OpenGL = OpenGL::V4_5;

pub const BG_COLOR: [f32; 4] = [0.08, 0.08, 0.08, 1.0];
pub const CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
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
pub const DEFAULT_CAMERA_LENGTH: f64 = 100.0;

pub const DT_BEFORE_UPDATE: f64 = 1.0; // in seconds
/// Camera moving speed
pub const SPEED: f64 = 45.0;
/// Camera moving speed
pub const SUPER_SPEED: f64 = 200.0;

pub const GRID_WIDTH: usize = WORLD_WIDTH as usize;
pub const GRID_HEIGHT: usize = WORLD_HEIGHT as usize;