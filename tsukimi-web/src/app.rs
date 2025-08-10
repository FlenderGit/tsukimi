use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

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

    async fn fetch_data() -> Result<String, String> {
        let response = reqwest::get("http://localhost:3000/engines")
            .await
            .map_err(|e| {
                // leptos::error!("Failed to fetch data: {}", e);
                e.to_string()
            })?;

        let text = response.text().await.map_err(|e| {
            // leptos::error!("Failed to read response text: {}", e);
            e.to_string()
        })?;
        Ok(text)
    }

    let async_data = LocalResource::new(move || {
        // This will be called when the resource is first created
        fetch_data()
    });

    view! {
        <button
            class="bg-primary hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            on:click=move |_| set_count.set(count.get() + 1)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>

        <table class="container mx-auto">
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Description"</th>
                    <th>"Version"</th>
                </tr>
            </thead>
            <tbody>
                {list.into_iter().map(|engine| view! {
                    <tr class="border-b">
                        <td>{engine.name}</td>
                        <td>{engine.description}</td>
                        <td>{engine.current_version}</td>
                    </tr>
                }).collect_view()}
            </tbody>
        </table>

        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <p>
                "Response from Google: "
                {move || {
                    match async_data.get() {
                        Some(Ok(data)) => data,
                        Some(Err(e)) => format!("Error: {}", e),
                        None => "Loading...".to_string(),
                    }
                }}
                </p>
        </Suspense>
    }
}
