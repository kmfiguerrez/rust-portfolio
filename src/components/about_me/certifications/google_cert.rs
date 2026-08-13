use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn GoogleCert() -> Element {
    rsx! {
        Card { class: "max-w-fit border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex items-center",
                img {
                    src: asset!("/assets/static/google-logo.svg"),
                    alt: "google logo",
                    class: "h-15 w-15",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "Google IT Support" }
                    p { class: "text-sm", "Google" }
                }
            }
        }
    }
}