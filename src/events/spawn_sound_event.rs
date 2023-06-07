use bevy::{audio::PlaybackSettings, ecs::event::Event};

#[derive(Event)]
pub struct SpawnSoundEvent {
    pub sound_path: String,
    pub playback_settings: PlaybackSettings,
}
