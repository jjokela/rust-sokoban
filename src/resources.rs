use ggez::event::KeyCode;
use specs::World;
use std::fmt::{self, Display};
use std::time::Duration;
use crate::audio::AudioStore;
use crate::events::Event;

// Resources
pub enum GameplayState {
    Playing,
    Won
}

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}