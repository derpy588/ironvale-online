# Ironvale Online

**Ironvale Online** is a multiplayer fantasy MMORPG built in **Rust**, powered by **Bevy** for rendering/gameplay and **Naia** for networking.  
It features a modular, server-authoritative architecture, procedurally generated worlds, rich lore, and both PvE and PvP elements.

---

## 🌍 Game Overview

Ironvale Online is set in a vast fantasy realm where players can explore diverse regions, fight monsters, gather resources, and engage in player-driven economies. The world blends handcrafted story arcs with procedural generation for replayability.

**Key Features:**
- **Persistent World** – Server-authoritative state with seamless multiplayer.
- **Handcrafted Content** – Unique world layouts on each server shard.
- **Deep Progression Systems** – Skills, crafting, equipment, and class archetypes.
- **Dynamic Combat** – Action-based combat with physics-driven interactions.
- **Social Gameplay** – Guilds, trade, large-scale PvP, and cooperative PvE.

---

## 🛠 Tech Stack

**Language:** Rust  
**Game Engine:** [Bevy](https://bevyengine.org/) (game logic, rendering)  
**Networking:** [Naia](https://github.com/naia-lib/naia) (client/server networking with entity sync)  
**Server Database:** PostgreSQL (persistent storage via `sqlx`)  
**Asset Pipeline:** `.glb` models, texture atlases, hot-reloading via Bevy’s asset system  
**Build System:** Cargo workspaces with modular crates for server, client, and tools  
**Testing:** `cargo test`, integration testing crates, load-testing tools

---

## 📂 Repository Layout

**Binary Crates:**

* `server` → Main game server; modules enabled via config (`gateway`, `jobs`, etc.)
* `client` → Game client for players
* `tools` → Developer utilities & test harnesses

**Library Crates:**

* Contain reusable logic (networking, combat, etc.)
* Linked by both client and server to avoid code duplication

---

## 🚀 Development Roadmap

**Phase 1 – Core Foundations**

1. **Setup Cargo Workspace** with shared crates (`shared`, `net`, `game`).
2. Implement **basic Naia networking**: connection, handshake, and heartbeat.
3. Add **ECS framework setup** in Bevy with minimal systems.
4. Create **placeholder assets** and hot-reload support.
5. Build minimal **server-client loop** with entity sync.

**Phase 2 – Game Simulation & World**

1. Implement **procedural world generation** (`world` crate).
2. Add **physics & collision** (Bevy + Rapier plugin).
3. Create **basic movement & camera** systems.
4. Implement **persistence layer** (`persistence` crate + PostgreSQL).

**Phase 3 – Combat & Interaction**

1. Add **combat systems** (server-authoritative hit detection).
2. Implement **NPC AI** for enemies and neutral creatures.
3. Introduce **inventory & item systems**.
4. Create **basic UI** (health, inventory, chat).

**Phase 4 – Multiplayer & Economy**

1. Enable **multiple server shards**.
2. Implement **trading, crafting, and guilds**.
3. Create **shared chat & messaging channels**.

**Phase 5 – Content & Polish**

1. Replace placeholder assets with production-quality models and textures.
2. Add **story arcs, quests, and unique regions**.
3. Optimize networking for large player counts.
4. Release **alpha version**.

---

## 🏗 Building & Running

### Prerequisites

* Rust stable (`rustup install stable`)
* PostgreSQL (for persistence)
* Git LFS (for large asset files)

### Build Commands

```bash
# Clone the repo
git clone https://github.com/yourusername/ironvale-online.git
cd ironvale-online

# Build all binaries
cargo build --workspace

# Run server
cargo run -p server

# Run client
cargo run -p client
```

---

## 📜 License

[MIT](LICENSE-MIT)
[APACHE](LICENSE-APACHE)

---

## 🤝 Contributing

We welcome pull requests! Currently know official way to make a pull request or guidelines but will take suggestions and help!
