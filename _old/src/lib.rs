mod bgr_color;
mod chroma_runner;
mod request_support;
// mod reqwest;

use std::time::Duration;

use bevy::time::common_conditions::on_timer;
use bgr_color::*;
use chroma_runner::*;

use bevy::log::*;
use bevy::prelude::{App, Color, IntoSystemConfig, Local, Plugin, ResMut};

pub struct ChromaPlugin;

impl Plugin for ChromaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ChromaRunnerPlugin::<&'static str>::default())
            .add_system(register)
            .add_system(set_green.run_if(on_timer(Duration::from_secs(1))));
    }
}

fn register(mut runner: ResMut<ChromaRunner<&'static str>>, mut has_ran_already: Local<bool>) {
    if !*has_ran_already {
        *has_ran_already = true;
        info!("system_update_chroma");
        runner.create_mouse_effect(
            "green",
            MouseRequest {
                effect: "CHROMA_STATIC",
                param: Into::<BGRColor>::into(Color::GREEN),
            },
        );
    }
}

fn set_green(mut runner: ResMut<ChromaRunner<&'static str>>) {
    runner.use_effect("green");
}
