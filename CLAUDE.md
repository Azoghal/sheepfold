# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo run          # Build and run (dev profile: opt-level 1, dynamic linking)
cargo run --release  # Optimised release build
cargo build        # Build without running
```

The `.cargo/config.toml` configures `clang`+`lld` for faster linking on x86_64 Linux.

## Architecture

**Sheepfold** is a 2D solar system simulation/visualisation built with [Bevy](https://bevyengine.org/) 0.18 and `bevy_egui`. It simulates fictional Mesopotamian-named celestial bodies orbiting a central star.

### File structure

| File | Contents |
|---|---|
| `main.rs` | Thin entry point — adds `DefaultPlugins` and `SolarSystemPlugin`, nothing else. |
| `solar_system.rs` | Everything: plugin definition, all components, resources, setup systems, UI, and simulation logic. |
| `orbit_material.rs` | `OrbitMaterial` + `OrbitMaterialPlugin` — custom `Material2d` for GPU orbit line rendering. |
| `debug_material.rs` | `DebugMaterialsPlugin` — debug/ring shader materials. |
| `units.rs` | `Kilometers` newtype and unit constants. |

### Plugin structure

`main.rs` wires up `SolarSystemPlugin` (defined in `solar_system.rs`), which in turn adds `OrbitMaterialPlugin`, `DebugMaterialsPlugin`, and `EguiPlugin`.

### Core systems

| System | Description |
|---|---|
| Orbital mechanics | `Orbiter` component holds radius, angular speed, and current angle. A `FixedUpdate` system advances positions each tick. |
| Time control | `OrbitRunner` resource controls pause state and timestep (5 levels: 1s → 1 week). Keyboard: `Space` pause, `,`/`.` speed. |
| Camera | Orthographic 2D with zoom (`-`/`=`). `CameraController` resource holds the scale; `apply_camera_scale` writes it to the projection each frame. Default view is 10 AU radius. |
| UI | `bevy_egui` panels for time, view, and debug controls. Screen-space `ScreenLabel` components link entity world positions to UI labels. |
| Orbit rendering | Custom `OrbitMaterial` rendered via WGSL shaders (`assets/shaders/orbit.wgsl`) using signed-distance fields to draw anti-aliased ellipses on the GPU. |

### Custom shaders

Three WGSL shaders live under `assets/shaders/`:
- `orbit.wgsl` — SDF-based ellipse with anti-aliasing (production orbit lines)
- `ring.wgsl` — debug ring visualisation
- `debug.wgsl` — UV coordinate debug view

Custom materials implement Bevy's `Material2d` trait. Shader uniforms are bound via `AsBindGroup` derive macros.

### Units

`units.rs` provides a `Kilometers` newtype and the `AU` constant (149,597,870.7 km). All distances in the simulation are in km internally; the 10 AU inner solar system radius is the default camera extent.
