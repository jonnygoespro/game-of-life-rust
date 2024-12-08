# Monorepo: Game of Life

This repository contains a monorepo setup for a Game of Life implementation, featuring:
- **Rust**: The core logic implemented in Rust, compiled to WebAssembly.
- **Frontend**: A JavaScript/TypeScript-based frontend to render and interact with the Game of Life.

## Folder Structure
```
/
|-- rust/               # Rust source code for the Game of Life logic
|-- frontend/           # Frontend project to render the Game of Life
```

---

## Prerequisites

Ensure you have the following installed:

### Rust and Wasm-Pack
- Install [Rust](https://www.rust-lang.org/).
- Install `wasm-pack`:
  ```bash
  cargo install wasm-pack
  ```

### Node.js and npm
- Install [Node.js](https://nodejs.org/) (which includes npm).

---

## Build and Run Instructions

### 1. Build the Rust Code
Navigate to the `rust/` folder and build the WebAssembly module:

```bash
cd rust/
wasm-pack build --target web --out-dir ../frontend/rust-dependencies
```

This compiles the Rust code into WebAssembly and places the output in the `frontend/rust-dependencies` directory.

### 2. Start the Frontend
Navigate to the `frontend/` folder and start the development server:

```bash
cd frontend/
npm install     # Install dependencies (only required the first time)
npm run start
```

This starts the frontend development server on `http://localhost:5173`.

---

## Usage
1. Open your browser and navigate to `http://localhost:5173`.
2. Interact with the Game of Life.

---

## Notes
- The Rust code is compiled into WebAssembly and dynamically imported by the frontend.
- Any changes to the Rust code require rebuilding with `wasm-pack`.
- The frontend leverages modern JavaScript tooling and expects Node.js v16+ for compatibility.
