use bevy::ecs::event::Event;

use super::spawn_sprite_event::SpawnSpriteEvent;

#[derive(Event)]
pub struct SpawnAnimatedSpriteEvent {
    pub tile_size: f32,
    pub tile_columns: usize,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub spawn_sprite_event: SpawnSpriteEvent,
}
