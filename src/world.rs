use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct TurnCounter(pub u8);
impl Default for TurnCounter {
    fn default() -> Self {
        TurnCounter(1)
    }
}
