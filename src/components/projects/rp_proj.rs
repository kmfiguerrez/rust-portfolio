use dioxus::prelude::*;

use crate::components::{
    card::{Card, CardContent, CardFooter},
    separator::Separator
};

use super::{proj_dialog::ProjectDialog, rp_dialog_content::RpDialogContent};

#[component]
pub fn RpProject(#[props(default)] class: String) -> Element {
    rsx! {
        Card { class: "max-md:mx-auto w-xs border-zinc-500! rounded-xl! bg-white! {class}",
            CardContent {
                div {
                    img {
                        class: "mx-auto",
                        src: asset!("/assets/static/dioxus_color.svg"),
                        alt: "IPv6 logo",
                        class: "h-30 w-30",
                    }
                }
                Separator { class: "mb-2", horizontal: true, decorative: true }
                div { class: "text-black",
                    h5 { class: "text-xl font-bold", "RUST PORTFOLIO" }
                    p { class: "text-sm", "#coding" }
                }
            }
            CardFooter { class: "justify-between",
                p { class: "text-blue-500 font-semibold", "Frontend App" }
                ProjectDialog { title: "RUST PORTFOLIO", demo_link: "", RpDialogContent {} }
            }
        }
    }
}

