# bevy_mod_chroma

Razer Chroma support plugin for Bevy. Currently only supports WASM.
Uses the Razer Chroma HTTP API to communicate with the Razer Chroma system.

## Usage

See [examples](https://github.com/datael/bevy_mod_chroma/tree/develop/examples) directory for more detailed examples.

General flow is that you need to register effects via `Chroma::create_effect` to get an `EffectHandle`, and then you can pass that handle to `Chroma::apply_effect` to apply.

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

> Note that the information you provide to `ChromaRunnerInitializationSettings` is displayed in the Razer Synapse connect menu!

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
