use bevy::prelude::*;
use bevy_mod_chroma::ChromaPlugin;
use bevy_mod_chroma_api::{Author, ChromaRunnerInitializationSettings, InitRequest};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ChromaPlugin::new(ChromaRunnerInitializationSettings::new(
            InitRequest {
                title: "Bevy Mod Chroma Examples",
                description: "Bevy Mod Chroma Examples",
                author: Author {
                    name: "Datael",
                    contact: "https://github.com/datael",
                },
                device_supported: vec![
                    "keyboard",
                    "mousepad",
                    "mouse",
                    "headset",
                    "keypad",
                    "chromalink",
                ],
                category: "application",
            },
        )))
        .run();
}
