use dioxus::prelude::*;

#[component]
pub fn Bagdes(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h4 { class: "text-blue-600 text-2xl mb-8", "BADGES" }
            div { class: "grid gap-y-4 lg:grid-cols-2",
                img { src: asset!("/assets/static/google-badge.png", AssetOptions::image().with_avif()) }
                div { class: "pl-2 lg:pt-4",
                    img { src: asset!("/assets/static/nb-badge.png", AssetOptions::image().with_avif()) }
                }
            }
        }
    }
}