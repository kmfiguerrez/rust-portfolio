use dioxus::prelude::*;

use crate::components::{
    card::{Card, CardContent, CardFooter},
    separator::Separator
};
use super::{
    proj_dialog::ProjectDialog,
    ipv6_dialog_content::IPv6DialogContent
};


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
                ProjectDialog {
                    title: "IPV6 SUBNETTING",
                    demo_link: "https://ipv6subnetting.netlify.app",
                    IPv6DialogContent {}
                }
            }
        }
    }
}

