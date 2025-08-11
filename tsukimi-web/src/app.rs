use leptos::*;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use tsukimi_core::models::Engine;

#[component]
pub fn App() -> impl IntoView {

    struct Route {
        name: &'static str,
        href: &'static str,
        icon: &'static str,
        sub_routes: Option<Vec<Route>>,
    }

    // When logged, subroute will be displayed
    let routes = vec![
        Route { name: "Home", href: "/", icon: "home_4_line", sub_routes: None },
        Route { name: "Visual novel", href: "/visual-novel", icon: "book_2_line", sub_routes: None },
        Route { name: "Engines", href: "/engine", icon: "engine_line", sub_routes: None },
    ];


    let view_routes = move || {
        routes.iter().map(|route| {
            let href = route.href.to_string();
            let name = route.name.to_string();
            let icon_class = format!("mgc_{} text-lg", route.icon);
            view! {
                <li>
                    <A href=href attr:class="flex items-center gap-2 p-2 rounded selectable">
                        <span class=icon_class></span>
                        {name}
                    </A>
                </li>
            }
        }).collect_view()
    };

    view! {
        <Router>
            <div class="flex min-h-screen">
                <nav class="p-2 w-64 bg-background-alt border-r border-border flex flex-col">
                <div class="flex items-center gap-2 p-2">
                    <img
                        src="/static/logo.png"
                        class="size-10 rounded-lg"
                        />
                    <p>Tsukimi</p>
                </div>
                    <ul class="flex flex-col gap-2 text-sm flex-1">
                        {view_routes}
                    </ul>
                    <div class="selectable flex items-center gap-2 p-2 rounded">
                        <img
                            src="/static/logo.png"
                            class="size-10 rounded-lg"
                            alt="Tsukimi Logo"
                        />
                        <div>
                            <p>"Flender"</p>
                            <p class="text-xs text-gray-500">"flender@tsukimi"</p>
                        </div>
                    </div>
                </nav>
                <main class="flex-1 p-4 mx-auto max-w-5xl">
                    <Routes fallback=move || view! { <p>"No route found"</p> }>
                        <Route path=path!("/") view=move || view! { <p>"Welcome to Tsukimi"</p> } />
                        // <ParentRoute path=path!("/engines") view=crate::components::engine::EngineList>
                            <Route path=path!("/engine/:id") view=crate::components::engine::EngineView />
                            <Route path=path!("/engine") view=crate::components::engine::EngineList />
                        // </ParentRoute>
                        <Route path=path!("/visual-novel/:id") view=crate::components::engine::EngineView />
                        <Route path=path!("/visual-novel") view=crate::components::visual_novel::VisualNovelList />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
