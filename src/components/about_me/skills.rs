use dioxus::prelude::*;

#[component]
pub fn Skills() -> Element {
    rsx! {
        div { id: "skills", class: "",
            h4 { class: "text-blue-600 text-xl", "SKILLS" }
        }
    }
}

