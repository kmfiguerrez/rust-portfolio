use dioxus::prelude::*;

#[component]
pub fn IPv6DialogContent() -> Element {
    rsx! {
        div { class: "h-87 overflow-y-auto",
            h5 { class: "font-semibold mb-2 underline underline-offset-4 decoration-sky-500 decoration-2",
                "DESCRIPTION"
            }
            p { class: "mb-4",
                "IPv6 Address subnetting is the process of splitting a network into smaller networks known as Subnets. 
                This project is a web app and can be used by anyone whether they're learning or practicing subnetting for free."
            }
            p { class: "mb-4",
                "I developed this project as one of my personal project to help me check my work or whenever I'm practicing IPv6 subnetting."
            }
            p { class: "mb-4",
                "This also useful to anyone who's going for the CCNA exam or any other equivalent certifications."
            }
            p { class: "mb-4",
                strong { "NOTE:" }
                " This app is on a free hosting plan so access to this app is limited."
            }

            h5 { class: "font-semibold mb-2 underline underline-offset-4 decoration-sky-500 decoration-2",
                "TECH USED"
            }
            ol { class: "list-disc list-inside mb-4",
                li { "HTML" }
                li { "Tailwind CSS" }
                li { "Gsap" }
                li { "Shadcn UI" }
                li { "Typescript" }
                li { "React.js" }
                li { "Next.js" }
            }

            h5 { class: "font-semibold mb-2 underline underline-offset-4 decoration-sky-500 decoration-2",
                "FEATURES"
            }
            ol { class: "list-disc list-inside mb-4",
                li { "Conversions" }
                li { "Validations" }
                li { "Generators" }
                li { "Utilities" }
            }

            p { class: "mb-4",
                strong { "NOTE:" }
                " I'm also going to write this in Dioxus."
            }
        }
    }
}