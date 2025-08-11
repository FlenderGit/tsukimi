use leptos::prelude::*;

#[slot]
pub struct TableHeader {
    children: ChildrenFn
}

#[component]
pub fn AsyncTable<T, F>(
    items: LocalResource<Result<Vec<T>, String>>,
    item_view: F,
    table_header: TableHeader,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> AnyView + 'static + Clone + Send + Sync,
{

    let inner_table = |data: LocalResource<Result<Vec<T>, String>>| {
        move || match data.get() {
            Some(Ok(list)) => {
                if list.is_empty() {
                    view! { <tr><td colspan=3 class="text-gray-500">{"No items found."}</td></tr> }.into_any()
                } else {
                    view! {
                        {list.into_iter()
                            .map(|t| item_view.clone()(t))
                            .collect_view()}
                    }.into_any()
                }
            },
            Some(Err(e)) => view! {
                <tr><td colspan=3 class="text-red-500">
                    {"Error fetching data: "}{e}
                </td></tr>
            }.into_any(),
            None => view! {
                <tr><td colspan=3>{"Loading 22..."}</td></tr>
            }.into_any(),
        }
    };

    view! {
        <table class="table-auto w-full">
            <thead class="text-left">
                <tr>
                    {(table_header.children)()}
                </tr>
            </thead>
            <tbody class="divide-y divide-gray-200 text-xs">
                <Transition
                    fallback=move || view! { <tr><td colspan=3>Loading 1...</td></tr> }.into_any()
                >
                    {inner_table(
                        items
                    )}
                </Transition>
            </tbody>
        </table>
    }
}

#[derive(Clone, Debug, serde::Serialize)]
struct PaginationParameters {
    query: String,
    page: u32,
    per_page: u32,
}

#[component]
pub fn Datatable<T, F>(
    url: reqwest::Url,
    table_header: TableHeader,
    item_view: F,
    marker: std::marker::PhantomData<T>,
) -> impl IntoView
where
    T: Clone + 'static + serde::de::DeserializeOwned,
    F: Fn(T) -> AnyView + 'static + Clone + Send + Sync,
{

    let (query, set_query) = signal(String::new());
    let (page, set_page) = signal(1);

    let data: LocalResource<Result<Vec<T>, String>> = LocalResource::new(move || {
        let url = url.clone();
        let t = PaginationParameters {
            query: query.get(),
            page: page.get(),
            per_page: 10, // You can adjust this value as needed
        };
        let pagination = serde_urlencoded::to_string(&t).unwrap_or_default();
        let url = url.join(&format!("?{}", pagination)).unwrap();
        async move {
            let client = reqwest::Client::new();
            let response = client.get(url).send().await.map_err(|e| e.to_string())?;
            if !response.status().is_success() {
                return Err(format!("Request failed with status: {}", response.status()));
            }
            response.json().await.map_err(|e| e.to_string())
        }
    });

    view! {
        <input
            type="text"
            placeholder="Search..."
            value=query
            on:input:target=move |e| set_query.set(e.target().value())
            class="p-1 border border-border rounded"
        />
        <AsyncTable
            items=data
            item_view=item_view
            table_header=table_header
        />
        <Pagination page set_page page_count=10 />
    }
}

#[component]
pub fn Pagination(
    page: ReadSignal<u32>,
    set_page: WriteSignal<u32>,
    page_count: u32,
) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center mt-4">
            <button
                on:click=move |_| set_page.update(|p| if *p > 1 { *p -= 1 })
                disabled={move || page.get() <= 1}
                class="px-4 py-2 bg-gray-200 rounded disabled:opacity-50"
            >
                "Previous"
            </button>
            <span>
                {move || format!("Page {} of {}", page.get(), page_count)}
            </span>
            <button
                on:click=move |_| set_page.update(|p| if *p < page_count { *p += 1 })
                disabled={move || page.get() >= page_count}
                class="px-4 py-2 bg-gray-200 rounded disabled:opacity-50"
            >
                "Next"
            </button>
        </div>
    }
}
