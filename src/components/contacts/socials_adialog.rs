use dioxus::prelude::*;

use crate::components::{
    alert_dialog::{AlertDialog, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel}
};

#[component]
pub fn SocialsAlertDialog(social_site: String, link: String, children: Element) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        button { onclick: move |_| open.set(true), class: "cursor-pointer", {children} }
        AlertDialog {
            class: "bg-zinc-800!",
            open: open(),
            on_open_change: move |v| open.set(v),
            AlertDialogTitle { "Visit {social_site}?" }
            AlertDialogDescription { "This link will take you to {social_site} website." }
            AlertDialogActions {
                AlertDialogCancel { "Stay!" }
                AlertDialogAction {
                    a { href: "{link}", target: "_blank", "Let's go!" }
                }
            }
        }
    }
}



