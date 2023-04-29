use bevy::prelude::*;
use bevy_mod_chroma::ChromaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ChromaPlugin)
        .run();
}
