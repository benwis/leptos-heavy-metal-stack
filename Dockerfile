# stage 1 - generate a rust recipe file for dependencies
FROM rustlang/rust:nightly as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 - build our dependencies
FROM rustlang/rust:nightly as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json

# install cargo-leptos
RUN cargo install cargo-leptos

RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 - use the main official docker image as builder
FROM rustlang/rust:nightly as builder

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs

# copy the app in docker image
COPY . /app

# set up working directory
WORKDIR /app

# copy dependecies
COPY --from=cacher /app/target /target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# target wasm32-unknown-unknown 
RUN rustup target add wasm32-unknown-unknown 

# install sass
RUN npm install -g sass

# set env variables for build

# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
# ENV LEPTOS_STYLE_FILE "style/main.scss"
# The browserlist https://browsersl.ist query used for optimizing the CSS.
ENV LEPTOS_BROWSERQUERY "defaults"

# build the app
RUN cargo leptos build --release

# use googles distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# copy app form builder
COPY --from=builder /app/target /app
WORKDIR /app

# Site .env parameters cargo-leptos
# ENV LEPTOS_SITE_ROOT "site"
# ENV LEPTOS_SITE_PKG_DIR "pkg"
# ENV LEPTOS_ASSETS_DIR "assets"
ENV LEPTOS_SITE_ADDRESS "0.0.0.0:3000"
EXPOSE "3000"

# start the application
CMD ["./server/release/leptos_start"]