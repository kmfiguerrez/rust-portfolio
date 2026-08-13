use dioxus::prelude::*;

use super::{languages::Languages, frameworks_tools::FrameworksTools, it_infrastructure::ItInfrastructure};

#[component]
pub fn Skills() -> Element {
    rsx! {
        div { id: "skills",
            // div { class: "flex items-center",
            //     img {
            //         src: asset!("/assets/static/stack.svg"),
            //         alt: "stack Logo",
            //         class: "h-20 w-20",
            //     }
            //     h4 { class: "text-blue-600 text-2xl font-semibold", "SKILLS" }
            // }
            h4 { class: "text-blue-600 text-2xl  mb-8", "SKILLS" }

            Languages { class: "mb-10" }
            FrameworksTools { class: "mb-10" }
            ItInfrastructure {}
        }
    }
}


