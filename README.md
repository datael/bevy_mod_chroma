# bevy_mod_chroma
Razer Chroma support plugin for Bevy. So far only tested in WASM.

## Usage

See [examples](https://github.com/datael/bevy_mod_chroma/tree/develop/examples) directory for usage.

General flow is that you need to register efffects, and then you can use them.

Create an instance of the ChromaPlugin and add it to your App, then you can use the Chroma parameter to create effects.

```rust
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
