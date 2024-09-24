//! Describe positions
//! 
//! If an object is not a 0 dimensioned object then the
//! position describe the origin of the object which is
//! at the top left hand corner of the object.

// use crate::consts::*;
use super::camera::Camera;

/// A **valid** position on the world, visible or not
// pub struct WorldPosition(pub f64, pub f64);
pub type WorldPosition = [f64; 2];

// impl WorldPosition {
    // pub fn is_valid(&self) -> bool {
        // self.0 >= 0.0 && self.1 >= 0.0 && self.0 <= WORLD_WIDTH && self.1 <= WORLD_HEIGHT
    // }
// }

// pub fn is_world_pos_valid(position: &WorldPosition) -> bool {
//     position[0] >= 0.0 && position[1] >= 0.0 && position[0] <= WORLD_WIDTH && position[1] <= WORLD_HEIGHT
// }

/// A **valid** position on the screen
// pub struct ScreenPosition(pub f64,pub f64);
pub type ScreenPosition = [f64; 2];


/// A position for the grid of cells
#[derive(PartialEq, Eq, Hash)]
pub struct GridPosition(pub usize, pub usize);

// impl ScreenPosition {
//     pub fn to_world_position(self, camera: &Camera) -> WorldPosition {
//         camera.translation(self)
//     }
// 
//     pub fn to_grid_position(self, camera: &Camera) -> GridPosition {
//         self.to_world_position(camera).to_grid_position()
//     }
// }

pub fn screen_to_world(position: ScreenPosition, camera: &Camera) -> WorldPosition {
    camera.screen_to_world(position)
}

pub fn screen_to_grid(position: ScreenPosition, camera: &Camera) -> GridPosition {
    world_to_grid(screen_to_world(position, camera))
}

// impl WorldPosition {
//     pub fn to_screen_position(self, camera: &Camera) -> Option<ScreenPosition> {
//         camera.projection(self)
//     }
// 
//     pub fn to_grid_position(self) -> GridPosition {
//         let (x,y) = (self.0, self.1);
//         GridPosition(x.trunc() as usize, y.trunc() as usize)
//     }
// }

pub fn world_to_screen(position: WorldPosition, camera: &Camera) -> Option<ScreenPosition> {
    camera.world_to_screen(position)
}

pub fn world_to_grid(position: WorldPosition) -> GridPosition {
    GridPosition(position[0].trunc() as usize, position[1].trunc() as usize)
}

impl GridPosition {
    pub fn to_world_position(self) -> WorldPosition {
        [self.0 as f64, self.1 as f64]
    }

    pub fn to_screen_position(self, camera: &Camera) -> Option<ScreenPosition> {
        world_to_screen(self.to_world_position(), camera)
    }
}