#[cfg(target_arch = "wasm32")]
mod wasi {
    use leptos_wasi::LeptosWasiOptions;
    use crate::app::App;

    pub fn serve_wasi() {
        // Configure for WASI
        let options = LeptosWasiOptions::builder()
            .leptos_options(get_leptos_options())
            .static_dir("static")
            .build();

        // Start the WASI server
        leptos_wasi::serve(options, App);
    }

    fn get_leptos_options() -> leptos::LeptosOptions {
        // Configure Leptos options for WASI
        leptos::LeptosOptions::builder()
            .output_name("ordo_lms")
            .site_root("public")
            .build()
    }
}

#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
pub fn main() {
    // Start WASI server when compiled for WebAssembly with SSR
    wasi::serve_wasi();
}