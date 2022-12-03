// use cfg_if::cfg_if;
// use leptos::*;
// use leptos_meta::*;
// use typed_builder::TypedBuilder;

// /// Properties for the [LiveReload] component.
// #[derive(TypedBuilder)]
// pub struct LiveReloadProps {
//     /// The URL at which the stylesheet can be located.
//     #[builder(setter(into))]
//     port: u16,
// }

// /// Injects an [HTMLLinkElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement) into the document
// /// head that loads a stylesheet from the URL given by the `href` property.
// ///
// /// ```
// /// use leptos::*;
// /// use leptos_meta::*;
// ///
// /// #[component]
// /// fn MyApp(cx: Scope) -> Element {
// ///   provide_context(cx, MetaContext::new());
// ///
// ///   view! { cx,
// ///     <main>
// ///       <Stylesheet href="/style.css"/>
// ///     </main>
// ///   }
// /// }
// /// ```
// #[allow(non_snake_case)]
// pub fn LiveReload(cx: Scope, props: LiveReloadProps) {
//     let LiveReloadProps { port } = props;
//    view!{cx,
//     <script crossorigin="">(function () {
//         var ws = new WebSocket('ws://127.0.0.1:3001/autoreload');
//         ws.onmessage = (ev) => {
//             console.log(`Reload message: `);
//             if (ev.data === 'reload') window.location.reload();
//         };
//         ws.onclose = () => console.warn('Autoreload stopped. Manual reload necessary.');
//     })()
//     </script>}
// }
