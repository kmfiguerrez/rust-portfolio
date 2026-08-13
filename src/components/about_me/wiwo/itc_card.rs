//! itc means IT certification.

use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn ItCard() -> Element {
    rsx! {
        Card { class: "md:max-w-[24rem] border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex flex-col",
                img {
                    src: asset!("/assets/static/it-cert.jpg", AssetOptions::image().with_avif()),
                    alt: "IT certification picture",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "IT Certifications" }
                    p { class: "text-sm", "#networking #coding" }
                }
            }
        }
    }
}