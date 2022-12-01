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

