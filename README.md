# Heavy Metal Stack
## Leptos stack with Axum, TailwindCSS, and Sqlite 

This example creates a basic todo app with an Axum backend that uses Leptos' server functions to call sqlx from the client and seamlessly run it on the server. It has an sqlite DB and can
be run with cargo leptos

## Install Cargo Leptos
```bash
cargo install --locked cargo-leptos
```
## Install TailwindCSS
See instructions [here](https://github.com/tailwindlabs/tailwindcss/releases).

## Run The Site
1. Build the Tailwind CSS
```bash
npx tailwindcss -i styles/tailwind.css -o static/styles/output.css --watch
```
2. In another window, run 
```bash
cargo leptos serve
```

## Run The Site in Dev Mode and Watch for Changes
1. Build the Tailwind CSS and Watch
```bash
npx tailwindcss -i styles/tailwind.css -o static/styles/output.css --watch
```
2. In another window, run 
```bash
cargo leptos watch
```

## Running without cargo-leptos

This template can still be used even without installing cargo-leptos, using the below process.

1. Build the Tailwind CSS
```bash
npx tailwindcss -i styles/tailwind.css -o static/styles/output.css --watch
```

2. Build the Client Side WASM
```bash
wasm-pack build --target=web --no-default-features --features=hydrate
```

3. Start the server to serve the content
```bash
cargo run --no-default-features --features=ssr
```

> You will have to rerun 2 if the client side WASM changes and 3 if the server code does. It might be safer to do both

## Sources
Setup Tailwind using the instructions from here:
https://github.com/gbj/leptos/discussions/125

