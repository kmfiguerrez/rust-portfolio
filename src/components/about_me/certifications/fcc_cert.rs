use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn FccCert() -> Element {
    rsx! {
        Card { class: "max-w-fit border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex items-center gap-x-1",
                img {
                    src: asset!("/assets/static/fcc-logo.svg"),
                    alt: "Freecodecamp logo",
                    class: "h-15 w-15",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "Backend Development & APIs" }
                    p { class: "text-sm", "Freecodecamp" }
                }
            }
        }
    }
}