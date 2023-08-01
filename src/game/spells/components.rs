use bevy::prelude::*;
use crate::game::utils::components::TeamType;

#[derive(Component)]
pub struct Spell {
    pub current_lifetime: f32,
    pub life_time: f32,
    pub damage: i32,
    //pub caster: Entity, //add radius
}

#[derive(Component)]
pub struct AutoCaster {
    pub cast_timer: Timer, //add architype
    pub team_type:TeamType
}

#[derive(Component)]
pub struct SpiralSpellMovement {
    pub start_pos: Vec2,
    pub angle_per_sec: f32,
    pub grow_dist_per_sec: f32,
}
