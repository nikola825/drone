## Flight controller implementation

Software needed - rust and cargo set up.
- `src` contains the flight controller code
- `embassy-vendored` contains the vendored code from the embassy project https://embassy.dev/
- `.vscode` contains settings for VS Code, the rust-analyzer settings from there can be applied to other development setups
  
You'll need an ST-Link programmer to flash this.

1. Default target is `stm32f411ce`. If you have a different chip, edit Cargo.toml, Embed.toml and .cargo/config.toml
2. Power the PCB and connect the programmer
3. `cargo run` or `cargo run --release`
