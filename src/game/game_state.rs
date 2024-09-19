use std::ops::Not;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Pause, // Can modify the grid
    Play // Each frame the game go to the next generation
}

impl From<bool> for GameState {
    fn from(value: bool) -> Self {
        if value {
            return GameState::Play;
        }
        GameState::Pause
    }
}

impl From<GameState> for bool {
    fn from(value: GameState) -> Self {
        match value {
            GameState::Play => true,
            GameState::Pause => false
        }
    }
}

impl Not for GameState {
    type Output = GameState;
    fn not(self) -> Self::Output {
        (!bool::from(self)).into()
    }
}