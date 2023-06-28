use bevy::prelude::*;

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct MouseInput;

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct GameEvents;

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct Cleanup;