use bevy::{prelude::App, DefaultPlugins};
use rust_bevy_chroma::ChromaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ChromaPlugin)
        .run();
}
