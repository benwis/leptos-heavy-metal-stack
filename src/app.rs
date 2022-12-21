use crate::routes::todo::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::new());
    view! {
        cx,
        <div>
        <Stylesheet id="leptos" href="/static/styles/output.css" />
            <Router>
                <header>
                    <h1>"My Tasks"</h1>
                </header>
                <main>
                <div>
                <Title text="Leptos Heavy Metal Stack"/>
                    <Routes>
                        <Route path="" element=|cx| view! {
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
