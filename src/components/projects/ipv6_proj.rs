use dioxus::prelude::*;

use crate::components::card::{Card, CardContent, CardFooter};
use crate::components::{separator::Separator};


#[component]
pub fn IPv6Project() -> Element {
    rsx! {
        Card { class: "max-md:mx-auto w-xs border-zinc-500! rounded-xl! bg-white!",
            CardContent {
                div {
                    img {
                        class: "mx-auto",
                        src: asset!("/assets/static/ipv6.svg"),
                        alt: "IPv6 logo",
                        class: "h-30 w-30",
                    }
                }
                Separator { class: "mb-2", horizontal: true, decorative: true }
                div { class: "text-black",
                    h5 { class: "text-xl font-bold", "IPV6 SUBNETTING" }
                    p { class: "text-sm", "#coding" }
                }
            }
            CardFooter { class: "justify-between",
                p { class: "text-blue-500 font-semibold", "Frontend App" }

                button {
                    img {
                        src: asset!("/assets/static/plus-circle-icon.svg"),
                        alt: "plus icon",
                        class: "h-8 w-8",
                    }
                }
            }
        }
    }
}

