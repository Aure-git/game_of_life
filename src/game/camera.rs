use crate::consts::{WORLD_WIDTH, WORLD_HEIGHT, SUPER_ZOOM_SPEED, ZOOM_SPEED, DEFAULT_CAMERA_WIDTH};

use super::{ScreenPosition, WorldPosition, DEFAULT_WINDOW_SIZE};

pub struct Camera {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    /// Shape of the drawable part of the screen
    draw_size: [f64; 2],
}

fn assert_camera(camera: &Camera) {
    // x inside world
    assert!(camera.x.is_sign_positive(), "camera.x < 0");
    assert!(camera.x < WORLD_WIDTH, "camera.x = {:?} >= WORLD_WIDTH", camera.x);
    // y inside world
    assert!(camera.y.is_sign_positive(), "camera.y < 0");
    assert!(camera.y < WORLD_HEIGHT, "camera.y = {:?} >= WORLD_HEIGHT", camera.y);
    // width > 0 and camera inside world width
    assert!(camera.width.is_sign_positive(), "camera.w < 0");
    assert!(camera.width >= f64::MIN_POSITIVE, "camera.w = {:?} too small", camera.width);
    assert!(camera.x + camera.width < WORLD_WIDTH, "camera out of bound, x={:?}, w={:?}", camera.x, camera.width);
    // height > 0 and camera inside world height
    assert!(camera.height.is_sign_positive(), "camera.h < 0");
    assert!(camera.height >= f64::MIN_POSITIVE, "camera.h = {:?} too small", camera.height);
    assert!(camera.y + camera.height < WORLD_HEIGHT, "camera out of bound, y={:?}, h={:?}", camera.y, camera.height);
    // screen width not absurd
    assert!(camera.draw_size[0].is_sign_positive(), "screen.w < 0");
    assert!(camera.draw_size[0] >= f64::MIN_POSITIVE, "screen.w too small");
    // screen height not absurd
    assert!(camera.draw_size[1].is_sign_positive(), "screen.h < 0");
    assert!(camera.draw_size[1] >= f64::MIN_POSITIVE, "screen.h too small");
    // camera and screen have same shape
    // sw / w = sh / h <=> sw / sh = w / h
    assert!(camera.height * camera.draw_size[0] - camera.draw_size[1] * camera.width < 0.001,
    "bad camera dimension, left={:?}, right={:?}",
    camera.height * camera.draw_size[0], camera.draw_size[1] * camera.width);
}

fn minimumf64(a: f64, b: f64) -> f64 {
    if a<b {a} else {b}
}

impl Default for Camera {
    fn default() -> Self {
        let world_center = [WORLD_WIDTH /2.0, WORLD_HEIGHT /2.0];
        let width = minimumf64(WORLD_WIDTH, DEFAULT_CAMERA_WIDTH);
        let height = width * DEFAULT_WINDOW_SIZE[1] / DEFAULT_WINDOW_SIZE[0];

        Camera {
            x: world_center[0] - width/2.0,
            y: world_center[1] - height/2.0,
            width,
            height,
            draw_size: DEFAULT_WINDOW_SIZE
        }
    }
}

impl Camera {

    pub fn debug_info(&self) {
        println!("DEBUG CAM: {:?}",[self.x, self.y, self.width, self.height, self.draw_size[0], self.draw_size[1]],);
    }

    fn correct_position(&mut self) {
        if self.x + self.width >= WORLD_WIDTH - 0.1 {
            self.x -= self.x + self.width - WORLD_WIDTH + 0.1;
        }
        if self.x.is_sign_negative() {
            self.x = 0.0;
        }
        else if self.x > WORLD_WIDTH {
            self.x = WORLD_WIDTH;
        }

        if self.y + self.height >= WORLD_HEIGHT - 0.1 {
            self.y -= self.y + self.height - WORLD_HEIGHT + 0.1;
        }
        if self.y.is_sign_negative() {
            self.y = 0.0;
        }
        else if self.y > WORLD_HEIGHT {
            self.y = WORLD_HEIGHT;
        }
    }

    fn correct_size(&mut self) {
        let mut need_to_check = false;

        if self.width < 1.0 {
            self.width = 1.0;
            need_to_check = true;
        }
        else if self.width > WORLD_WIDTH - 0.1 {
            self.x = 0.0;
            self.width = WORLD_WIDTH - 0.1;
            need_to_check = true;
        }

        if self.height < 1.0 {
            self.height = 1.0;
            need_to_check = true;
        }
        else if self.height > WORLD_HEIGHT - 0.1 {
            self.y = 0.0;
            self.height = WORLD_HEIGHT - 0.1;
            need_to_check = true;
        }

        if need_to_check {
            self.correct_dimension();
        }
    }

    fn correct_dimension(&mut self) {
        // sw / w = sh / h
        let a = self.width * self.draw_size[1];
        let b = self.height * self.draw_size[0];

        if a > b {
            self.width = self.height * self.draw_size[0] / self.draw_size[1];
        }
        else if a < b {
            self.height = self.width * self.draw_size[1] / self.draw_size[0];
        }
    }

    /// Update the shape of the camera to match the screen
    pub fn resize(&mut self, new_draw_size: [f64;2]) {

        // Camera and screen should have same shape
        // sw / w = sh / h
        self.width *= new_draw_size[0] / self.draw_size[0];
        self.height = self.width * new_draw_size[1] / new_draw_size[0];

        self.draw_size = new_draw_size;

        // Check and move camera inside the world
        self.correct_size();
        self.correct_position();

        assert_camera(&self);
    }

    pub fn world_to_screen(&self, position: WorldPosition) -> Option<ScreenPosition> {
        // top left
        let x = (position[0] - self.x) / self.width * self.draw_size[0];
        let y = (position[1] - self.y) / self.height * self.draw_size[1];

        if (x.is_sign_positive() && x < self.draw_size[0]) || (y.is_sign_positive() && y < self.draw_size[1]) {
            Some([x,y])
        }
        else {
            let a = (position[0] +1.0 - self.x) / self.width * self.draw_size[0];
            let b = (position[1] +1.0 - self.y) / self.height * self.draw_size[1];

            if a.is_sign_positive() || b.is_sign_positive() {
                Some([x,y])
            }
            else {
                None
            }
        }
    }

    pub fn screen_to_world(&self, position: ScreenPosition) -> WorldPosition {
        [
            position[0] * self.width / self.draw_size[0] + self.x,
            position[1] * self.height / self.draw_size[1] + self.y
        ]
    }

    pub fn move_max(&mut self, transform: [f64;2]) {
        self.x += transform[0];
        self.y += transform[1];

        self.correct_position();

        assert_camera(&self);
    }

    pub fn zoom(&mut self, dt: f64, faster: bool) {
        if self.width > 1.0 && self.height > 1.0 {
            let epsilon = (1.0 + dt) * if faster {SUPER_ZOOM_SPEED} else {ZOOM_SPEED};
            let new_width = self.width - 2.0*epsilon;
            if new_width >= f64::MIN_POSITIVE {
    
                let new_height = self.height * new_width / self.width; // w/h = nw/nh <=> nh = h*nw/w
                if new_height >= f64::MIN_POSITIVE {
    
                    self.x += epsilon;
                    self.y += (self.height - new_height) /2.0;
                    self.width = new_width;
                    self.height = new_height;
                }
            }
    
            assert_camera(&self);
        }
    }

    pub fn dezoom(&mut self, dt: f64, faster: bool) {
        if self.width < WORLD_WIDTH - 0.1 && self.height < WORLD_HEIGHT - 0.1 {
            let epsilon = (1.0 + dt) * if faster {SUPER_ZOOM_SPEED} else {ZOOM_SPEED};
            let new_width = self.width + 2.0*epsilon;
            let new_height = self.height * new_width / self.width; // w/h = nw/nh <=> nh = h*nw/w
            self.x -= epsilon;
            self.y -= (new_height - self.height) / 2.0;
            self.width = new_width;
            self.height = new_height;
    
            self.correct_size();
            self.correct_position();
    
            assert_camera(&self);
        }
    }

    pub fn cell_lenght(&self) -> f64 {
        self.draw_size[0] / self.width
    }
}