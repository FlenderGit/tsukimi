use leptos::{prelude::*, svg::path};
use leptos_router::components::*;
use leptos_router::path;
use tsukimi_core::models::Engine;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let (query, set_query) = signal(String::new());

    let list = vec![
        tsukimi_core::models::Engine {
            id: Uuid::from_bytes([0; 16]),
            name: "Engine One".to_string(),
            current_version: "1.0.1".to_string(),
            description: "First engine in the list.".to_string(),
        },
        tsukimi_core::models::Engine {
            id: Uuid::from_bytes([0; 16]),
            name: "Engine Two".to_string(),
            current_version: "1.0.2".to_string(),
            description: "Second engine in the list.".to_string(),
        },
    ];

    async fn fetch_data(query: String) -> Result<Vec<Engine>, String> {
        let response = reqwest::get("http://localhost:3000/engine?query=".to_owned() + &query)
            .await
            .map_err(|e| {
                // leptos::error!("Failed to fetch data: {}", e);
                e.to_string()
            })?;

        let text = response.json().await.map_err(|e| {
            // leptos::error!("Failed to parse response: {}", e);
            e.to_string()
        })?;

        Ok(text)
    }

    let async_data = LocalResource::new(move || fetch_data(query.get()));

    fn inner_table(data: LocalResource<Result<Vec<Engine>, String>>) -> impl IntoView {
        // On construit d'abord `rows` avec le même type dans chaque branche
        move || match data.get() {
            Some(Ok(list)) => {
                if list.is_empty() {
                    return view! {
                        <tr>
                            <td colspan=3 class="text-gray-500">{"No engines found."}</td>
                        </tr>
                    }
                    .into_any();
                }

                view! {
                    {list.into_iter()
                        .map(|engine| {
                            view! {
                                <tr class="border-b">
                                    <td>{engine.name}</td>
                                    <td>{engine.description}</td>
                                    <td>{engine.current_version}</td>
                                </tr>
                            }
                        })
                        .collect_view()
                    }
                }
                .into_any()
            }
            Some(Err(e)) => view! {
                <tr>
                    <td colspan=3 class="text-red-500">
                        {"Error fetching data: "}{e}
                    </td>
                </tr>
            }
            .into_any(),
            None => view! {
                <tr>
                    <td colspan=3>Loading...</td>
                </tr>
            }
            .into_any(),
        }
    }

    view! {
        <Router>
            <div class="flex min-h-screen">
                <nav class="p-4 w-64 bg-background-alt border-r border-border">
                    <ul class="text-white">
                        <li><a href="/" class="hover:underline">"Home"</a></li>
                        <li><a href="/about" class="hover:underline">"About"</a></li>
                        <li><a href="/contact" class="hover:underline">"Contact"</a></li>
                    </ul>
                </nav>
                <main class="flex-1 p-4">
                    <Routes fallback=move || view! { <p>"No route found"</p> }>
                        <Route path=path!("/") view=move || view! { <p>"Welcome to Tsukimi"</p> } />
                    </Routes>
                </main>
            </div>
        </Router>

        // <button
        //     class="bg-primary hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        //     on:click=move |_| set_count.set(count.get() + 1)
        // >
        //     "Click me: "
        //     {count}
        // </button>
        // <p>
        //     "Double count: "
        //     {move || count.get() * 2}
        // </p>

        // <input
        //     type="text"
        //     class="border rounded px-2 py-1"
        //     placeholder="Search..."
        //     value=query
        //     on:input:target=move |e| set_query.set(e.target().value())
        // />

        // <table class="container mx-auto">
        //     <thead>
        //         <tr>
        //             <th>"Name"</th>
        //             <th>"Description"</th>
        //             <th>"Version"</th>
        //         </tr>
        //     </thead>
        //     <tbody>
        //         <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        //             {inner_table(async_data)}
        //         </Suspense>
        //     </tbody>
        // </table>

    }
}
