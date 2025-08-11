use leptos::*;
use tsukimi_core::models::Engine;
use leptos::prelude::*;
use leptos_router::{hooks::use_params_map, components::A};
use crate::components::utils::*;

#[component]
pub fn VisualNovelList() -> impl IntoView {
    view! {
        <p>"Example"</p>
        <div class="card">
                <Datatable
                    url=reqwest::Url::parse("http://localhost:3000/visual-novel").unwrap()
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
                            </tr>
                        }.into_any()
                    }
                    marker={std::marker::PhantomData::<Engine>}
                >
                    <TableHeader slot>
                        <th>"Name"</th>
                    </TableHeader>
                </Datatable>
            </div>
    }
}
