# bevy_mod_chroma

[![Crates.io](https://img.shields.io/crates/v/bevy_mod_chroma)](https://crates.io/crates/bevy_mod_chroma)
![License](https://img.shields.io/github/license/datael/bevy_mod_chroma)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
<!--[![docs.rs](https://docs.rs/bevy_mod_chroma/badge.svg)](https://docs.rs/bevy_mod_chroma)-->

Razer Chroma support plugin for Bevy. Uses the Razer Chroma HTTP API to communicate with the Razer Chroma system.

## Usage

You will need Razer Chroma software installed to properly use this plugin.<br />
If you don't have certain types of hardware devices, [Razer also provides an emulator](https://github.com/razerofficial/ChromaEmulator) that you can use to test your effects.

General flow is that you need to register effects via `Chroma::create_effect` to get an `EffectHandle`, and then you can pass that handle to `Chroma::apply_effect` to apply.

See the [examples](https://github.com/datael/bevy_mod_chroma/tree/develop/examples) directory for more detailed examples.

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_mod_chroma::{
    Author, Category, ChromaPlugin, ChromaRunnerInitializationSettings, InitRequest, SupportedDevice,
};

fn main() {
    App::new()
        .add_plugin(ChromaPlugin::new(ChromaRunnerInitializationSettings::new(
            InitRequest {
                title: "Your Bevy app title goes here",
                description: "Your Bevy app description goes here",
                author: Author {
                    name: "Your name",
                    contact: "Your contact",
                },
                device_supported: vec![
                    SupportedDevice::Keyboard,
                    SupportedDevice::Mouse,
                    //...
                ],
                category: Category::Application, // or Category::Game
            },
        )))
        //...
        .run();
}
```

> NOTE: The information you provide to `ChromaRunnerInitializationSettings` is displayed in the Razer Synapse connect menu!

### Create and apply effects

Use the `Chroma` parameter in systems to create and apply effects:

```rust
use bevy_mod_chroma::{Chroma, Effect, EffectHandle, MouseEffect};

fn red_mouse(mut chroma: Chroma) {
    let red_handle = chroma.create_effect(Effect::Mouse(MouseEffect::Static {
        color: Color::RED.into(),
    }));

    chroma.apply_effect(&red_handle);
}
```

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release.

Compatibility of `bevy_mod_chroma` versions:

| `bevy_mod_chroma` branch | Compatible Bevy version |
| ------------------------ | ----------------------- |
| `develop`                | `0.11`                  |
| `bevy_0.10`              | `0.10`                  |

## TODOs
* Documentation
* Error handling
* Retries
* Reconnect after dropped connections

## Wants
* Websocket version
* Built-in animations support
