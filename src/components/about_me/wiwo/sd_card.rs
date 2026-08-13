//! sd means software development.

use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn SdCard() -> Element {
    rsx! {
        Card { class: "md:max-w-[24rem] border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex flex-col",
                img {
                    src: asset!("/assets/static/web-dev.jpg", AssetOptions::image().with_avif()),
                    alt: "software development picture",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "Software Development" }
                    p { class: "text-sm", "#desktop #web #mobile" }
                }
            }
        }
    }
}