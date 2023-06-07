use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct DespawnEntityEvent {
    pub entity: Entity,
}
