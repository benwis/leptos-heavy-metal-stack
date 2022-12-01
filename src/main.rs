use cfg_if::cfg_if;
use leptos::*;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        routing::{post},
        error_handling::HandleError,
        Router,
    };
    use std::net::SocketAddr;
    use crate::todo::*;
    use leptos_heavy_metal_stack::*;
    use http::StatusCode;
    use tower_http::services::ServeDir;

    #[tokio::main]
    async fn main() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8082));
        log::debug!("serving at {addr}");

        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

        let mut conn = db().await.expect("couldn't connect to DB");
        sqlx::migrate!("./migrations")
            .run(&mut conn)
            .await
            .expect("could not run SQLx migrations");

        crate::todo::register_server_functions();

        // These are Tower Services that will serve files from the static and pkg repos.
        // HandleError is needed as Axum requires services to implement Infallible Errors
        // because all Errors are converted into Responses
        let static_service = HandleError::new( ServeDir::new("./static"), handle_file_error);
        let pkg_service = HandleError::new( ServeDir::new("./pkg"), handle_file_error);

        /// Convert the Errors from ServeDir to a type that implements IntoResponse
        async fn handle_file_error(err: std::io::Error) -> (StatusCode, String) {
            (
                StatusCode::NOT_FOUND,
                format!("File Not Found: {}", err),
            )
        }


        // build our application with a route
        let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .nest_service("/pkg", pkg_service)
        .nest_service("/static", static_service)
        .fallback(leptos_axum::render_app_to_stream("todo_app_sqlite_axum", |cx| view! { cx, <App/> }));

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    // client-only stuff for Trunk
    else {
        use leptos_heavy_metal_stack::todo::*;

        pub fn main() {
            console_error_panic_hook::set_once();
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(|cx| {
                view! { cx, <App/> }
            });
        }
    }
}
