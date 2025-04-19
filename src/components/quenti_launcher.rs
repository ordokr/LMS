use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn QuentiLauncher() -> impl IntoView {
    let (launching, set_launching) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);

    let launch_quenti = move |_| {
        set_launching.set(true);
        set_error.set(None);
        set_success.set(None);

        spawn_local(async move {
            let args = JsValue::from_serde(&serde_json::json!({})).unwrap();
            
            match invoke("launch_quenti_app", args).await {
                Ok(result) => {
                    let result_str = result.as_string().unwrap_or_default();
                    set_success.set(Some(result_str));
                },
                Err(e) => {
                    let error_str = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                    set_error.set(Some(error_str));
                }
            }
            
            set_launching.set(false);
        });
    };

    view! {
        <div class="quenti-launcher">
            <h2 class="launcher-title">Quenti Quiz Module</h2>
            <p class="launcher-description">
                Launch the Quenti Quiz Module as a standalone application.
            </p>
            
            <button 
                class="launcher-button"
                on:click=launch_quenti
                disabled=launching.get()
            >
                {move || if launching.get() { "Launching..." } else { "Launch Quenti" }}
            </button>
            
            {move || error.get().map(|err| view! {
                <div class="launcher-error">
                    <p>{err}</p>
                </div>
            })}
            
            {move || success.get().map(|msg| view! {
                <div class="launcher-success">
                    <p>{msg}</p>
                </div>
            })}
        </div>
    }
}
