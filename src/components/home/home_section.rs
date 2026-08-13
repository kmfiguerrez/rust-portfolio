use dioxus::prelude::*;

#[component]
pub fn HomeSection() -> Element {
    rsx! {
        section {
            id: "home",
            class: "h-screen flex flex-col justify-center container-x",
            img {
                class: "h-20 w-20",
                src: asset!("/assets/static/k-logo.png", AssetOptions::image().with_avif()),
            }
            p { class: " text-3xl  font-bold mb-4 pl-1 sm:text-4xl lg:text-7xl",
                "KARL MICHAEL FIGUERREZ"
            }
            p { class: "pl-1 sm:text-lg md:text-xl lg:text-3xl",
                "Network"
                span { class: "text-blue-500", " Enginner" }
                " & "
                span { class: "text-amber-500", "Rust" }
                " Developer"
            }
        }
    }
}