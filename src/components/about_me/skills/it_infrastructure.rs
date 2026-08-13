use dioxus::prelude::*;

#[component]
pub fn ItInfrastructure(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h5 { class: "mb-4", "Networking" }

            div { class: "grid grid-cols-2 pl-4 gap-y-7 lg:grid-cols-3 lg:w-1/2",
                // Swtiching
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/switch.svg"),
                        alt: "network switch logo",
                        class: "h-8 w-8",
                    }
                    span { "Switching" }
                }
                // Router
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/router.svg"),
                        alt: "network router logo",
                        class: "h-8 w-8",
                    }
                    span { "Routing" }
                }
                // Wireless router
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/wireless-router.svg"),
                        alt: "docker logo",
                        class: "h-7 w-7",
                    }
                    span { "WLANs" }
                }
                // Windows
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/windows.svg"),
                        alt: "windows os logo",
                        class: "h-7 w-7",
                    }
                    span { "Windows" }
                }
                // Linux
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/linux.svg"),
                        alt: "linux logo",
                        class: "h-8 w-8",
                    }
                    span { "Linux" }
                }
            }
        }
    }
}