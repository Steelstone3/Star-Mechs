use bevy::prelude::{App, Plugin};

use crate::events::{
    despawn_entity_event::DespawnEntityEvent,
    spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sound_event::SpawnSoundEvent,
    spawn_sprite_event::SpawnSpriteEvent, user_interface_event::UserInterfaceEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnAnimatedSpriteEvent>()
            .add_event::<SpawnSpriteEvent>()
            .add_event::<DespawnEntityEvent>()
            .add_event::<SpawnSoundEvent>()
            .add_event::<UserInterfaceEvent>();
    }
}
