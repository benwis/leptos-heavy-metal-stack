use crate::routes::todo::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <div>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/static/styles/output.css" />
            <Router>
                <header>
                    <h1>"My Tasks"</h1>
                </header>
                <main>
                <div>
                <Title text="Leptos Heavy Metal Stack"/>
                    <Routes>
                        <Route path="" view=|cx| view! {
                            cx,
                            <Todos/>
                        }/>
                    </Routes>
                    </div>
                </main>
            </Router>
        </div>
    }
}
