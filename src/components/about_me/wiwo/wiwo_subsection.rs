use dioxus::prelude::*;

use super::{sd_card::SdCard, itc_card::ItCard};

#[component]
pub fn Wiwo(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h4 { class: "text-blue-600 mb-8 text-lg md:text-2xl", "WHAT I'M WORKING ON" }
            div { class: "grid gap-y-4 lg:grid-cols-2",
                SdCard {}
                ItCard {}
            }
        }
    }
}