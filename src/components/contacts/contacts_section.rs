use dioxus::prelude::*;

use super::form::MessageForm;

#[component]
pub fn ContactsSection() -> Element {
    rsx! {
        section { id: "contacts", class: "h-screen border container-x",
            h3 { class: "text-center font-bold mb-16 text-3xl md:text-5xl", "CONTACTS" }
            div { class: "grid gap-y-10 md:grid-cols-2",
                div {
                    h4 { class: "text-lg text-blue-500 mb-8 md:text-2xl", "SOCIALS" }
                    div { class: "h-75 sm:w-95 sm:mx-auto",
                        img {
                            class: "mx-auto",
                            src: asset!("/assets/static/socials.svg"),
                            alt: "burger menu logo",
                            loading: "lazy",
                        }
                    }
                }
                div {
                    h4 { class: "text-lg text-blue-500 mb-8 md:text-2xl", "SEND ME A MESSAGE" }
                    MessageForm {}
                }
            }
        }
    }
}