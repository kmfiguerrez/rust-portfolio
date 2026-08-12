use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "fixed w-full",
            // div { class: "border flex justify-between  sm:px-8 xl:mx-11 2xl:max-w-[1500px] 2xl:mx-auto",
            div { class: "border flex justify-between container-x py-4",
                div { "KMF" }
                div {
                    ul { class: "flex gap-x-3",
                        li {
                            a { href: "#home", "HOME" }
                        }
                        li {
                            a { href: "#about-me", "ABOUT ME" }
                        }
                        li {
                            a { href: "#projects", "PROJECTS" }
                        }
                        li {
                            a { href: "#contacts", "CONTACTS" }
                        }
                    }
                }
            }
        }

    }
}