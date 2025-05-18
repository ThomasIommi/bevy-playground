# Breakout

A simple clone of the classic **Breakout** game.

This project was created as a learning exercise to gain hands-on experience with:
- Game development concepts
- The Rust programming language
- The [Bevy Engine](https://bevyengine.org/)

## Running with Dynamic Linking

For faster iteration during development, you can run the application using **dynamic linking** for the Bevy Engine. 
This reduces compile times and allows for quicker feedback loops.

### Option 1: Run with Cargo

Use the following command to enable Bevy’s dynamic linking feature:

```bash
cargo run --features bevy/dynamic_linking
```

### Option 2: Modify Cargo.toml

Alternatively, you can add Bevy with the dynamic_linking feature directly to your dependencies:

```bash
cargo add bevy -F dynamic_linking
```

**Note**: remember to revert back the _Cargo.toml_ for the final release.