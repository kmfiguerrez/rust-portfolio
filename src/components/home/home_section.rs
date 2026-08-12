use dioxus::prelude::*;

#[component]
pub fn HomeSection() -> Element {
    rsx! {
        section {
            id: "home",
            class: "h-screen border flex flex-col justify-center container-x",
            "NOCTIS"
        }
    }
}