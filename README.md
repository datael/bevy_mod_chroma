# bevy_mod_chroma

[![Crates.io](https://img.shields.io/crates/v/bevy_mod_chroma)](https://crates.io/crates/bevy_mod_chroma)
![License](https://img.shields.io/github/license/datael/bevy_mod_chroma)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
<!--[![docs.rs](https://docs.rs/bevy_mod_chroma/badge.svg)](https://docs.rs/bevy_mod_chroma)-->

Razer Chroma support plugin for Bevy. Uses the Razer Chroma HTTP API to communicate with the Razer Chroma system.

Currently only supports WASM.

## Usage

You will need Razer Chroma hardware and software installed to use this plugin.<br />
If you don't have all types of devices, [Razer also provides an emulator](https://github.com/razerofficial/ChromaEmulator) that you can use to test your effects.

General flow is that you need to register effects via `Chroma::create_effect` to get an `EffectHandle`, and then you can pass that handle to `Chroma::apply_effect` to apply.

See the [examples](https://github.com/datael/bevy_mod_chroma/tree/develop/examples) directory for more detailed examples.

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_mod_chroma::{Author, ChromaPlugin, ChromaRunnerInitializationSettings, InitRequest};

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
                    "keyboard",
                    "mouse",
                    //...
                ],
                category: "Category of your Bevy app goes here",
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

## TODOs
* Documentation
* Expand API support to include all devices
* Error handling
* Retries
* Reconnect after dropped connections

## Wants
* Websocket version
* Built-in animations support
