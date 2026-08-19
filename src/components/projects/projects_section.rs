use dioxus::prelude::*;

use super::{ipv6_proj::IPv6Project, ipv4_proj::IPv4Project, rp_proj::RpProject};

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            id: "projects",
            class: "mb-32 min-h-fit lg:mb-0 lg:h-screen container-x",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "PROJECTS" }
            div { class: "grid gap-y-4 lg:grid-cols-2 lg:justify-items-center xl:grid-cols-3",
                RpProject { class: "justify-self-center-safe" }
                IPv6Project { class: "justify-self-center-safe" }
                IPv4Project { class: "justify-self-center-safe" }
            }
        }
    }
}