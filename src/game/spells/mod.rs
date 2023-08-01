pub mod components;
mod systems;
mod spell_factory;

use bevy::prelude::*;
use systems::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_hammer_auto_caster_spell)
            .add_systems(Update, (update_spell_duration, synch_damage_area_to_position))
            .add_systems(Update, (update_spell_casting_hammer, update_spiral_spell).after(update_spell_duration));
    }
}
