use dioxus::prelude::*;

use super::{ipv6_proj::IPv6Project, ipv4_proj::IPv4Project};

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            id: "projects",
            class: "max-md:mb-32 max-md:min-h-fit md:h-screen container-x",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "PROJECTS" }
            div { class: "grid gap-y-4 lg:grid-cols-2 lg:justify-items-center",
                IPv6Project {}
                IPv4Project {}
            }
        }
    }
}