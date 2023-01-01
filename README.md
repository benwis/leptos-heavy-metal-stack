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

## Server Side Rendering with cargo-leptos
cargo-leptos is now the easiest and most featureful way to build server side rendered apps with hydration. It provides automatic recompilation of client and server code, wasm optimisation, CSS minification, and more! Check out more about it [here](https://github.com/akesson/cargo-leptos)

0. Build the Tailwind CSS
```bash
npx tailwindcss -i styles/tailwind.css -o static/styles/output.css --watch
```
1. Install cargo-leptos
```bash
cargo install --locked cargo-leptos
``` 
2. Build the site in watch mode, recompiling on file changes
```bash
cargo leptos watch
```
3. When ready to deploy, run
```bash
cargo leptos build --release
```

## Server Side Rendering without cargo-leptos
To run it as a server side app with hydration, you'll need to have wasm-pack installed.

0. Edit the `[package.metadata.leptos]` section and set `site-root` to `"pkg"`. You'll also want to change the path of the `<StyleSheet / >` component in the root component to point towards the CSS file in the root. This tells leptos that the WASM/JS files generated by wasm-pack are available at `./pkg` and that the CSS files are no longer processed by cargo-leptos. Building to alternative folders is not supported at this time.
1. Install wasm-pack
```bash
cargo install wasm-pack
```
2. Build the Webassembly used to hydrate the HTML from the server
```bash
wasm-pack build --target=web --debug --no-default-features --features=hydrate
```
3. Run the server to serve the Webassembly, JS, and HTML 
```bash
cargo run --no-default-features --features=ssr
```
## Sources
Setup Tailwind using the instructions from here:
https://github.com/gbj/leptos/discussions/125

