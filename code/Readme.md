## Flight controller and configurator tool

This folder contains the flight_controller project to be flashed to the hardware in this repo.
It also contains the configurator GUI tool to configure motor mapping and directions of the flight controller.

### Contents
- `flight_controller` - Rust crate using the [Embassy](https://embassy.dev) implementing the flight control code
  - Cargo.toml contains feature flags for various variants of the FC PCB
  - Can be flashed through STM32 Cube programmer, dfu-util or probe-rs
- `configurator` - gui tool for configuring the motor mapping and directions
- `common` - stuff shared between configurator and FC
- `.vscode` and `.zed` - configurations for Rust analyzer for VSCode and Zed editors
