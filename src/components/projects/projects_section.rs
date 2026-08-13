use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section { id: "projects", class: "h-screen border container-x",
            h3 { class: "text-center font-bold text-5xl", "PROJECTS" }
        }
    }
}