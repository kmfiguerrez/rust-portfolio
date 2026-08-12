use dioxus::prelude::*;

use super::skills::Skills;

#[component]
pub fn AboutMeSection() -> Element {
    rsx! {
        section { id: "about-me", class: "h-screen border container-x",
            h3 { class: "text-center", "About Me" }
            Skills {}
        }
    }
}