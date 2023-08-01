use super::components::*;
use super::spell_factory::*;
use crate::game::utils::components::*;
use bevy::prelude::*;

use std::time::Duration;

pub fn update_spell_duration(
    mut spell_query: Query<(Entity, &mut Spell)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (entity, mut spell) in spell_query.iter_mut() {
        spell.current_lifetime += dt;

        if spell.current_lifetime > spell.life_time {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_spell_casting_hammer(
    mut auto_cast_query: Query<(Entity, &mut AutoCaster, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut auto_caster, transform) in auto_cast_query.iter_mut() {
        auto_caster.cast_timer.tick(time.delta());
        if auto_caster.cast_timer.finished() {
            spawn_spell_hammer(entity, transform.translation, &mut commands, &asset_server);
        }
    }
}

pub fn update_spiral_spell(
    mut spiral_query: Query<(&mut Transform, &SpiralSpellMovement, &Spell)>,
) {
    for (mut transform, spiral, spell) in spiral_query.iter_mut() {
        let dist: f32 = spell.current_lifetime * spiral.grow_dist_per_sec;
        let angle: f32 = spell.current_lifetime * spiral.angle_per_sec;

        let angle_vec: Vec2 = Vec2::from_angle(angle);
        let pos: Vec2 = spiral.start_pos + angle_vec * dist;

        transform.translation = Vec3 {
            x: pos.x,
            y: pos.y,
            z: 0.0,
        };
    }
}

pub fn synch_damage_area_to_position(mut damage_area_query: Query<(&Transform, &mut DamageArea)>) {
    for (transform, mut damage_area) in damage_area_query.iter_mut() {
        damage_area.position = transform.translation;
    }
}

pub fn create_hammer_auto_caster_spell(mut commands: Commands) {
    commands.spawn((
        AutoCaster {
            cast_timer: Timer::new(Duration::from_secs_f32(0.75), TimerMode::Repeating),
            team_type: TeamType::Hero,
        },
        Transform { //link as a child to player
            translation: Vec3::ZERO,
            ..default()
        },)
    );
}
