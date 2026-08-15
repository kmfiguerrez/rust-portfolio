use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut open = use_signal(|| false);
    let filename = if open() {"x-close"} else {"burger-menu"};
    let icon_path = format!("{}/{}.svg", asset!("/assets/static"), filename);

    rsx! {
        nav {
            class: "fixed w-full bg-zinc-900 bg-linear-to-b from-zinc-400/30",
            class: if open() { "min-h-fit" }, // If open, display the links vertically.
            div { class: "flex justify-between items-center container-x sm:py-4",
                // logo
                div { class: "text-xl", "KMF" }
                // mobile navigation button
                div { class: "sm:hidden",
                    button { onclick: move |_| open.toggle(),
                        img {
                            class: "mx-auto",
                            src: icon_path,
                            alt: "{filename} logo",
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
            div {
                class: "overflow-hidden sm:hidden",
                class: if !open() { "hidden" },
                class: if open() { "visible" },
                ul { class: "flex flex-col gap-y-1 text-zinc-400 mb-4",
                    li {
                        a {
                            class: "block w-full active:bg-zinc-700",
                            href: "#home",
                            "HOME"
                        }
                    }
                    li {
                        a {
                            class: "block w-full active:bg-zinc-700",
                            href: "#about-me",
                            "ABOUT ME"
                        }
                    }
                    li {
                        a {
                            class: "block w-full active:bg-zinc-700",
                            href: "#projects",
                            "PROJECTS"
                        }
                    }
                    li {
                        a {
                            class: "block w-full active:bg-zinc-700",
                            href: "#contacts",
                            "CONTACTS"
                        }
                    }
                }
            }
        }

    }
}