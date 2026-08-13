use dioxus::prelude::*;

#[component]
pub fn Languages(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h5 { class: "mb-4", "Languages" }

            div { class: "grid grid-cols-2 pl-4 gap-y-7 lg:grid-cols-3 lg:w-1/2 ",
                // HTML
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/html5.svg"),
                        alt: "HTML 5 logo",
                        class: "h-8 w-8",
                    }
                    span { "HTML" }
                }
                // CSS
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/css.svg"),
                        alt: "css logo",
                        class: "h-8 w-8",
                    }
                    span { "CSS" }
                }
                // JS
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/javascript.svg"),
                        alt: "javascript logo",
                        class: "h-8 w-8",
                    }
                    span { "Javascript" }
                }
                // TS
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/typescript.svg"),
                        alt: "typescript logo",
                        class: "h-8 w-8",
                    }
                    span { "Typescript" }
                }
                // Rust
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/rust-orange.svg"),
                        alt: "rust logo",
                        class: "h-8 w-8",
                    }
                    span { "Rust" }
                }
                // SQL
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/sql.svg"),
                        alt: "sql logo",
                        class: "h-8 w-8",
                    }
                    span { "SQL" }
                }
            }
        }
    }
}