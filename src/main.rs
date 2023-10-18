use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

mod level;
mod player;

use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_plugins((LevelPlugin, PlayerPlugin))
        .run();
}
