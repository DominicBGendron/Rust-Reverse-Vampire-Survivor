use bevy::prelude::*;
use game::GamePlugin;

pub mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GamePlugin)
        .run();
}
