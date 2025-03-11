use bevy::prelude::*;

#[derive(Resource)]
pub struct TurnCounter(pub u8);
impl Default for TurnCounter {
    fn default() -> Self {
        TurnCounter(1)
    }
}

#[derive(Component)]
pub struct StatusUI;

#[derive(Resource)]
pub struct Status(pub String);
impl Default for Status {
    fn default() -> Self {
        Status("O's turn".to_string())
    }
}
