# Bevy Modding Experiment

This project demonstrates a simple approach to adding modding capabilities to a Bevy game, inspired by [bevy_dynamic_plugin](https://docs.rs/bevy_dynamic_plugin/latest/bevy_dynamic_plugin/index.html).

## Limitations

- **Version Consistency:** Both the main application and plugins must use the same Bevy version and dynamically link Bevy using the `bevy/dynamic_linking` feature.
- **Component Sharing:** Components intended for use by plugins must reside in a separate dynamic library (dylib) crate and should not be subdirectory of the main application (?).

## Getting Started

To run the project, follow these steps:

```bash
git clone https://github.com/goktug97/bevy_modding_experiment
cd bevy_modding_experiment/plugin
cargo build --release
cd ../bevy_modding
cargo run --release
```

## How It Works

- **Main Application:** The core application initializes a scene featuring a camera and a stationary circle. It dynamically loads the plugin shared library using [libloading](https://docs.rs/libloading/latest/libloading/). Then it calls the `setup` function, passing a mutable reference to the `App` instance (`&mut App`). This function is a required implementation within the plugin.

- **Plugin:** The plugin is responsible for implementing the `setup` function. In this example, the plugin adds a system to move the circle.
