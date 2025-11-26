# Fantasy Craft

> The lightweight, data-oriented Rust game engine built for the Web.

![Fantasy Craft Banner](https://placehold.co/1200x300/1a1a1a/ff6b00?text=Fantasy+Craft&font=montserrat)

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

[![Rust](https://img.shields.io/badge/Rust-1.75+-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Macroquad](https://img.shields.io/badge/Macroquad-0.4-C63527)](https://macroquad.rs/)
[![Hecs](https://img.shields.io/badge/Hecs-ECS-005571)](https://github.com/Ralith/hecs)
[![Parry2d](https://img.shields.io/badge/Parry2d-Physics-blueviolet)](https://github.com/dimforge/parry)
[![WebAssembly](https://img.shields.io/badge/WASM-Ready-654FF0?logo=webassembly&logoColor=white)](https://webassembly.org/)

**Fantasy Craft** is a 2D/2.5D game engine designed for developers who love **Rust** but hate bloat.

Unlike heavy general-purpose engines, Fantasy Craft focuses on a "code-first" approach. It combines the immediate-mode rendering simplicity of **Macroquad** with a high-performance Entity Component System (**Hecs**) and robust physics (**Parry2d**).

It is the official engine of the **Foxvoid Ecosystem**, meaning it comes with built-in hooks for instant cloud deployment, save states, and leaderboards.

---

## 🌟 Key Features

* **🦀 100% Rust:** Type-safe, memory-safe, and blazingly fast.
* **🕸️ WASM First:** Optimized to produce tiny binaries (~200kb) that load instantly in any browser.
* **🧱 Data-Oriented Architecture:** Built on top of `hecs`, a lightweight and fast ECS (Entity Component System).
* **💥 Physics Ready:** Integrated `parry2d` for precise collision detection and spatial queries.
* **☁️ Foxvoid Cloud Ready:** Native integration with Foxvoid Platform for:
    * Cloud Storage (Saves/Configs)
    * Leaderboards
    * Auto-updater via the Foxvoid Launcher.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fantasy-craft = "0.1.0"
```

## 🚀 Quick Start

Here is how to create a simple window with a player entity in less than 30 lines of code.

```rust
  // todo: add fantasy craft code example
```

## 🏗️ Architecture

Fantasy Craft stands on the shoulders of giants. We believe in composing the best crates in the ecosystem rather than reinventing the wheel.

| Layer | Technology | Description |
| :--- | :--- | :--- |
| **Rendering** | `macroquad` | Cross-platform windowing and immediate mode graphics. |
| **ECS** | `hecs` | A lightweight, archetypal ECS for managing game state. |
| **Physics** | `parry2d` | Geometric queries and collision detection. |
| **Networking** | `reqwest` (WASM) | HTTP client for talking to the Foxvoid Backend. |

## 📄 License

Fantasy Craft is licensed under the Mozilla Public License 2.0 (MPL-2.0).

What does this mean for you?

   ✅ Commercial Use: You can use this engine to build and sell closed-source games. You keep 100% of your game code rights.

   🔄 Modifications: If you modify the source code of the engine itself, you must share those modifications back to the community.

See LICENSE for more details.

<div align="center">
  <sub>Part of the <a href="https://github.com/foxvoid-studio">Foxvoid Studio</a> ecosystem.</sub> 
</div>
