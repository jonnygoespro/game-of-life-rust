# Stage 1: Build Rust WebAssembly
FROM rust:latest as rust-builder

RUN cargo install wasm-pack
WORKDIR /app/rust
COPY ./rust /app/rust
RUN wasm-pack build --target web --out-dir ../frontend/rust-dependencies

# Stage 2: Build Frontend
FROM node:18 as frontend-builder

WORKDIR /app/frontend

COPY ./frontend/package*.json ./
RUN npm install vite
RUN npm ci
COPY ./frontend ./
COPY --from=rust-builder /app/frontend/rust-dependencies /app/frontend/rust-dependencies
RUN npm run build

# Stage 3: Serve Application
FROM nginx:alpine as production

COPY --from=frontend-builder /app/frontend/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
