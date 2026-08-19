use dioxus::prelude::*;

#[component]
pub fn RpDialogContent() -> Element {
    rsx! {
        div { class: "h-87 overflow-y-auto",
            h5 { class: "font-semibold mb-2 underline underline-offset-4 decoration-sky-500 decoration-2",
                "DESCRIPTION"
            }
            p { class: "mb-4",
                "This project is single page application portfolio built with Dioxus and Rust
                and you're looking right at it."
            }
            p { class: "mb-4",
                "I included this portfolio in the projects section just to show I know Dioxus and Rust lol!"
            }

            h5 { class: "font-semibold mb-2 underline underline-offset-4 decoration-sky-500 decoration-2",
                "TECH USED"
            }
            ol { class: "list-disc list-inside mb-4",
                li { "Dioxus" }
                li { "Rust" }
                li { "Tailwind" }
            }
        }
    }
}