use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "fixed w-full bg-zinc-900 bg-linear-to-b from-zinc-400/30",
            // div { class: "border flex justify-between  sm:px-8 xl:mx-11 2xl:max-w-[1500px] 2xl:mx-auto",
            div { class: "flex justify-between container-x py-4",
                div { class: "text-xl", "KMF" }
                div {
                    ul { class: "flex gap-x-3 text-zinc-400",
                        li {
                            a { class: "hover:text-zinc-200", href: "#home", "HOME" }
                        }
                        li {
                            a {
                                class: "hover:text-zinc-200",
                                href: "#about-me",
                                "ABOUT ME"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-zinc-200",
                                href: "#projects",
                                "PROJECTS"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-zinc-200",
                                href: "#contacts",
                                "CONTACTS"
                            }
                        }
                    }
                }
            }
        }

    }
}