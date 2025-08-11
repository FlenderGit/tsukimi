use leptos::*;
use leptos::prelude::*;
use leptos_router::{hooks::use_params_map, components::A};
use tsukimi_core::models::Engine;
use crate::components::utils::*;

#[component]
pub fn EngineList() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
        <div class="card">
            <h1 class="text-2xl font-bold">"Engines"</h1>
            <p>
                "This page lists all available engines. Click on an engine to view its details."
            </p>
            <p>If you want to contribute, you can add a new engine by clicking the button below.</p>
        </div>
        <div class="card">
                <Datatable
                    url=reqwest::Url::parse("http://localhost:3000/engine").unwrap()
                    item_view=|engine: Engine| {
                        view! {
                            <tr class="selectable hover:bg-gray-100">
                                <td>
                                    <A
                                        href=format!("{}", engine.id)
                                        attr:class="text-blue-500 hover:underline"
                                    >
                                        {engine.name}
                                    </A>
                                </td>
                                <td>{engine.description}</td>
                                <td>
                                    <A
                                        href={format!("{}#versions", engine.id)}
                                        attr:class="text-blue-500 hover:underline"
                                    >
                                        {engine.current_version}
                                    </A>
                                </td>
                            </tr>
                        }.into_any()
                    }
                    marker={std::marker::PhantomData::<Engine>}
                >
                    <TableHeader slot>
                        <th>"Name"</th>
                        <th>"Description"</th>
                        <th>"Current Version"</th>
                    </TableHeader>
                </Datatable>
            </div>
        </div>
    }
}

#[component]
pub fn EngineView(
) -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id");
    view! {
        <div class="engine-view">
            <h1>"Engine View"</h1>
            <p>{
                match id() {
                    Some(id) => format!("Viewing engine with ID: {}", id),
                    None => "No engine ID provided.".to_string(),
                }
            }</p>
        </div>
    }
}
