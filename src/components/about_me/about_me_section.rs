use dioxus::prelude::*;

use super::{wiwo::Wiwo, certifications::Certifications, badges::Bagdes, skills::Skills};

#[component]
pub fn AboutMeSection() -> Element {
    rsx! {
        section { id: "about-me", class: "min-h-fit container-x mb-32",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "ABOUT ME" }
            Wiwo { class: "mb-16" }
            Certifications { class: "mb-16" }
            Bagdes { class: "mb-16" }
            Skills {}
        }
    }
}