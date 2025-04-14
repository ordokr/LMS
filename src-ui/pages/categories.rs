use leptos::*;
use leptos_router::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Category {
    id: i64,
    name: String,
    description: Option<String>,
}

async fn fetch_categories() -> Result<Vec<Category>, reqwasm::Error> {
    let url = "/api/categories"; // Assuming API is mounted at /api
    reqwasm::http::Request::get(url)
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn CategoriesPage(cx: Scope) -> impl IntoView {
    let categories = create_resource(cx, || (), |_| async move { fetch_categories().await });

    view! { cx,
        <main>
            <h1>"Categories"</h1>
            <Suspense fallback=view! { cx, <p>"Loading categories..."</p> }>
                {move || categories.read(cx).map(|result| match result {
                    Ok(cats) => view! { cx,
                        <ul>
                            {cats.into_iter().map(|category| {
                                view! { cx,
                                    <li>
                                        <A href=format!("/categories/{}", category.id)>
                                            {&category.name}
                                        </A>
                                        {category.description.map(|d| view! { cx, <p>{d}</p> })}
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view(cx),
                    Err(e) => view! { cx, <p>"Error loading categories: " {e.to_string()}</p> }.into_view(cx),
                })}
            </Suspense>
        </main>
    }
}
