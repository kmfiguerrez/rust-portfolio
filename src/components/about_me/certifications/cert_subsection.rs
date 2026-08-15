use dioxus::prelude::*;

use super::{google_cert::GoogleCert, fcc_cert::FccCert, nb_cert::NetworkBasicsCert};

#[component]
pub fn Certifications(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "{class}",
            h4 { class: "text-blue-600 mb-8 text-lg md:text-2xl", "CERTIFICATIONS" }
            div { class: "grid gap-y-4 lg:grid-cols-2 lg:gap-y-16",
                GoogleCert {}
                FccCert {}
                NetworkBasicsCert {}
            }
        
        }
    }
}