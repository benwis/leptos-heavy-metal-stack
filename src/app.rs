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
            <Router>
                <header>
                    <h1>"My Tasks"</h1>
                </header>
                <main>
                    <Routes>
                        <Route path="" element=|cx| view! {
                            cx,
                            <Todos/>
                        }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
