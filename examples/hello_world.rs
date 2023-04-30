use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_chroma::ChromaPlugin;
use bevy_mod_chroma_api::{
    api::{Effect, MouseEffect},
    Author, Chroma, ChromaRunnerInitializationSettings, EffectHandle, InitRequest,
};

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
        .add_startup_system(create_effects)
        .add_system(cycle_effects.run_if(on_timer(Duration::from_secs_f32(0.1))))
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct EffectLibrary(HashMap<&'static str, EffectHandle>);

fn create_effects(mut commands: Commands, mut chroma: Chroma) {
    let red_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::RED.into(),
    }));
    let green_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::GREEN.into(),
    }));
    let blue_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::BLUE.into(),
    }));

    let mut effect_library = HashMap::<&'static str, EffectHandle>::default();
    effect_library.insert("red", red_handle);
    effect_library.insert("green", green_handle);
    effect_library.insert("blue", blue_handle);

    commands.insert_resource(EffectLibrary(effect_library));
}

fn cycle_effects(mut chroma: Chroma, effect_library: Res<EffectLibrary>, mut counter: Local<u8>) {
    let effect = match *counter {
        0 => "red",
        1 => "green",
        2 => "blue",
        _ => unimplemented!(),
    };

    *counter = (*counter + 1_u8) % 3;

    chroma.apply_effect(&effect_library.get(effect).unwrap());
}
