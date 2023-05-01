use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};
use bevy_mod_chroma::{
    Author, Chroma, ChromaPlugin, ChromaRunnerInitializationSettings, Effect, EffectHandle,
    InitRequest, MouseEffect,
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
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
        .insert_resource(EffectLibrary::default())
        .add_startup_system(create_effects)
        .add_system(cycle_effects.run_if(on_timer(Duration::from_secs_f32(0.25))))
        .run();
}

#[derive(Resource, Deref, DerefMut, Default)]
struct EffectLibrary(HashMap<&'static str, EffectHandle>);

fn create_effects(mut chroma: Chroma, mut effect_library: ResMut<EffectLibrary>) {
    let red_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::RED.into(),
    }));
    let green_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::GREEN.into(),
    }));
    let blue_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::BLUE.into(),
    }));

    effect_library.insert("red", red_handle);
    effect_library.insert("green", green_handle);
    effect_library.insert("blue", blue_handle);
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
