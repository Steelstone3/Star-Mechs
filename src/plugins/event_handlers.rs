use crate::events::event_handlers::{
    despawn_entity::despawn_entity, spawn_animated_sprite::spawn_animated_sprite,
    spawn_sound::spawn_sound, spawn_sprite::spawn_sprite,
};
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_animated_sprite,
                spawn_sprite,
                despawn_entity,
                spawn_sound,
            ),
        );
    }
}
