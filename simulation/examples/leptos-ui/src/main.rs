use leptos::*;

mod app;
mod components;
mod signals;
mod utils;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App/> });
}