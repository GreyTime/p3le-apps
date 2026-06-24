# p3le-apps

My personal hub of small web apps, built with **Rust** — **Yew** (WASM) on the frontend, **Rocket** on the backend.

## Apps

| App | Status |
|-----|--------|
| 🏃 **Habit Tracker** | placeholder |
| ✅ **Todo List** | placeholder |
| 🎲 **Hobby Chooser** | placeholder |
| 🔌 **Cable Database** | placeholder |
| 🎯 **Goal Tracker** | [planned](GOAL_TRACKER_PLAN.md) |

## Stack

- **Frontend:** [Yew](https://yew.rs/) (Rust → WASM), [Trunk](https://trunkrs.dev/) for building
- **Backend:** [Rocket](https://rocket.rs/) — serves the frontend build and will host API endpoints
- **Workspace:** Cargo workspace with `frontend/` and `backend/` crates

## Development

```sh
# Serve frontend (hot-reloads on changes)
trunk serve frontend/index.html

# Build frontend & serve via Rocket
trunk build frontend/index.html
cargo run -p backend
```
