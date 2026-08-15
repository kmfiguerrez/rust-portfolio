use dioxus::prelude::*;

#[component]
pub fn Bagdes(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h4 { class: "text-blue-600 mb-8 text-lg md:text-2xl", "BADGES" }
            div { class: "grid gap-y-4 md:grid-cols-2",
                img {
                    class: "max-sm:mx-auto",
                    src: asset!("/assets/static/google-badge.png", AssetOptions::image().with_avif()),
                }
                div { class: "pl-2 lg:pt-4",
                    img {
                        class: "max-sm:mx-auto",
                        src: asset!("/assets/static/nb-badge.png", AssetOptions::image().with_avif()),
                    }
                }
            }
        }
    }
}