use dioxus::prelude::*;

use crate::components::{
    dialog::{Dialog, DialogTitle},
    separator::Separator,
    alert_dialog::{AlertDialog, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel}
};

#[css_module("/src/components/dialog/style.css")]
struct Styles;

#[component]
pub fn ProjectDialog(title: String, demo_link: String, children: Element) -> Element {
    let mut dialog_open = use_signal(|| false);
    let mut alert_dialog_open = use_signal(|| false);

    rsx! {
        button { onclick: move |_| dialog_open.set(true),
            img {
                src: asset!("/assets/static/plus-circle-icon.svg"),
                alt: "plus icon",
                class: "h-8 w-8 animate-pulse",
            }
        }
        Dialog {
            open: dialog_open(),
            on_open_change: move |v| dialog_open.set(v),
            class: "bg-white! text-black! text-left!",
            button {
                class: "{Styles::dx_dialog_close} text-black!",
                r#type: "button",
                aria_label: "Close",
                tabindex: if dialog_open() { "0" } else { "-1" },
                onclick: move |_| dialog_open.set(false),
                "x"
            }
            DialogTitle { class: "text-black!", "{title}" }
            Separator { horizontal: true, decorative: true }
            // Dialog content
            {children}
            // Dialog footer
            Separator { horizontal: true, decorative: true }
            div { class: "flex justify-center gap-x-7",
                // Code link.
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/code.svg"),
                        alt: "code icon",
                        class: "h-5 w-5",
                    }
                    button {
                        onclick: move |_| alert_dialog_open.set(true),
                        class: "hover:font-semibold",
                        "Code"
                    }
                    AlertDialog {
                        open: alert_dialog_open(),
                        on_open_change: move |v| alert_dialog_open.set(v),
                        AlertDialogTitle { "Want the code?" }
                        AlertDialogDescription { "Contact me thru my social medias." }
                        AlertDialogActions {
                            AlertDialogAction { "Nah!" }
                            AlertDialogCancel { "Sure!" }
                        }
                    }
                }
                // Demo link.
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/demo.svg"),
                        alt: "demo icon",
                        class: "h-5 w-5",
                    }
                    a {
                        href: "{demo_link}",
                        target: "_blank",
                        class: "hover:font-semibold",
                        "Demo"
                    }
                }
            }
        }
    }
}