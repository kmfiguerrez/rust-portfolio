use dioxus::prelude::*;

#[component]
pub fn FrameworksTools(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h5 { class: "mb-4", "Frameworks & Tools" }

            div { class: "grid grid-cols-2 pl-4 gap-y-7 lg:grid-cols-3 lg:w-1/2",
                // HTML
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/dioxus_color.svg"),
                        alt: "Dioxus logo",
                        class: "h-8 w-8",
                    }
                    span { "Dioxus" }
                }
                // Tailwind
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/tailwind.svg"),
                        alt: "tailwind logo",
                        class: "h-8 w-8",
                    }
                    span { "Tailwind" }
                }
                // Docker
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/docker.svg"),
                        alt: "docker logo",
                        class: "h-8 w-8",
                    }
                    span { "Docker" }
                }
                // GIT
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/git.svg"),
                        alt: "git logo",
                        class: "h-8 w-8",
                    }
                    span { "GIT" }
                }
                // Github
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/github.svg"),
                        alt: "github logo",
                        class: "h-8 w-8",
                    }
                    span { "Github" }
                }
            }
        }
    }
}