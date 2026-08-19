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
    let mut code_open = use_signal(|| false);
    let mut demo_open = use_signal(|| false);

    rsx! {
        button { onclick: move |_| dialog_open.set(true),
            img {
                src: asset!("/assets/static/plus-circle-icon.svg"),
                alt: "plus icon",
                class: "h-8 w-8 cursor-pointer animate-pulse",
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
            div { class: "flex justify-center gap-x-8",
                // Code link.
                div { class: "flex items-center gap-x-2",
                    img {
                        src: asset!("/assets/static/code.svg"),
                        alt: "code icon",
                        class: "h-5 w-5",
                    }
                    button {
                        onclick: move |_| code_open.set(true),
                        class: "font-semibold opacity-70 hover:opacity-100 cursor-pointer",
                        "Code"
                    }
                    AlertDialog {
                        class: "bg-zinc-800!",
                        open: code_open(),
                        on_open_change: move |v| code_open.set(v),
                        AlertDialogTitle { "View the code?" }
                        AlertDialogDescription { "Contact me thru my social media." }
                        AlertDialogActions {
                            AlertDialogCancel { "Nah!" }
                            AlertDialogAction { "Sure!" }
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
                    button {
                        onclick: move |_| demo_open.set(true),
                        class: "font-semibold opacity-70 hover:opacity-100 cursor-pointer",
                        "Demo"
                    }
                    AlertDialog {
                        class: "bg-zinc-800!",
                        open: demo_open(),
                        on_open_change: move |v| demo_open.set(v),
                        AlertDialogTitle { "View the app?" }
                        AlertDialogDescription { "This link will take you to the app's website." }
                        AlertDialogActions {
                            AlertDialogCancel { "Stay!" }
                            AlertDialogAction {
                                a { href: "{demo_link}", target: "_blank", "Let's go!" }
                            }
                        }
                    }
                }
            }
        }
    }
}