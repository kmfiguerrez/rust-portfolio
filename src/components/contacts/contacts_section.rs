use dioxus::prelude::*;

#[component]
pub fn ContactsSection() -> Element {
    rsx! {
        section { id: "contacts", class: "h-screen border container-x",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "CONTACTS" }
        }
    }
}