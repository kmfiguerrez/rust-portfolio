use dioxus::prelude::*;

use super::ipv6_proj::IPv6Project;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section { id: "projects", class: "h-screen border container-x",
            h3 { class: "text-center font-bold text-5xl mb-16", "PROJECTS" }
            IPv6Project {}
        }
    }
}