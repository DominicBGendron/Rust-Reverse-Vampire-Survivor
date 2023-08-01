use bevy::prelude::*;
use super::components::*;

use crate::game::utils::components::*;
use std::f32::consts::PI;

pub fn spawn_spell_hammer(owner: Entity, spawn_pos: Vec3, commands: &mut Commands, asset_server: &Res<AssetServer>){

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: spawn_pos,
                ..default()
            },
            texture: asset_server.load("sprites/hammer.png"),
            ..default()
        },
        Spell {
            damage: 1,
            life_time: 5.0,
            current_lifetime: 0.0,
        },
        SpiralSpellMovement {
            grow_dist_per_sec: 20.0,
            start_pos: Vec2::new(spawn_pos.x, spawn_pos.y),
            angle_per_sec: 2.5 * PI,
        },
        Rotator {
            angle: 0.0,
            speed: 40.0,
        },
        DamageArea {
            area: 10.0,
            damage: 5,
            onwer_team_type: TeamType::Hero,
            position: Vec3::ZERO,
            owner : owner
        },
    ));
}