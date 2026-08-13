use dioxus::prelude::*;

#[component]
pub fn ContactsSection() -> Element {
    rsx! {
        section { id: "contacts", class: "h-screen border container-x",
            h3 { class: "text-center text-5xl font-bold", "CONTACTS" }
        }
    }
}