use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "fixed w-full bg-zinc-900 bg-linear-to-b from-zinc-400/30",
            // div { class: "border flex justify-between  sm:px-8 xl:mx-11 2xl:max-w-[1500px] 2xl:mx-auto",
            div { class: "flex justify-between items-center container-x sm:py-4",
                // logo
                div { class: "text-xl", "KMF" }
                // mobile navigation button
                div { class: "sm:hidden",
                    button {
                        img {
                            class: "mx-auto",
                            src: asset!("/assets/static/burger-menu.svg"),
                            alt: "burger menu logo",
                            class: "h-8 w-8",
                        }
                    }
                }
                // links
                div { class: "max-sm:hidden",
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