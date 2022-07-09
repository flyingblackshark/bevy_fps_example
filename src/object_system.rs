use bevy::prelude::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;

pub struct ObjectSystemPlugin;


#[derive(Debug, Component, Copy, Clone, Eq, PartialEq, Reflect)]
pub struct HealthStatus{
   pub hp:u32
}

impl Plugin for ObjectSystemPlugin {
    fn build(&self, app: &mut App) {

    }
}