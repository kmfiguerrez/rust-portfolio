use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn NetworkBasicsCert() -> Element {
    rsx! {
        Card { class: "max-w-fit border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex items-center gap-x-2",
                img {
                    src: asset!("/assets/static/cisco-logo1.svg"),
                    alt: "cisco logo",
                    class: "h-15 w-15",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "Networking Basics" }
                    p { class: "text-sm", "Cisco Networking Academy" }
                }
            }
        }
    }
}