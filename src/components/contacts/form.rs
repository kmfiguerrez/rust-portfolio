use dioxus::{logger::tracing, prelude::*};

use crate::components::{
    input::Input,
    label::Label,
    textarea::{Textarea, TextareaVariant}, button::Button,
    alert_dialog::{AlertDialog, AlertDialogTitle, AlertDialogDescription, AlertDialogActions, AlertDialogAction, AlertDialogCancel}
};

#[component]
pub fn MessageForm() -> Element {
    // For Alert Dialog
    let mut open = use_signal(|| false);


    let handle_submit = move |e: Event<FormData>| {
        // prevent_default stops the browser from navigating the page on submit
        // Also from realoading the page.
        e.prevent_default();
        tracing::info!("{:?}", e.values());
        open.set(true);

    };

    rsx! {
        form { onsubmit: handle_submit,
            div { class: "mb-8",
                Label { class: "mb-1 text-base!", html_for: "name", "Name" }
                Input {
                    id: "name",
                    name: "name",
                    required: true,
                    placeholder: "Enter your name",
                    class: "w-full",
                }
            }
            div { class: "mb-8",
                Label { class: "mb-1 text-base!", html_for: "email", "Email" }
                Input {
                    id: "email",
                    name: "email",
                    required: true,
                    r#type: "email",
                    placeholder: "Enter your email",
                    class: "w-full",
                }
            }
            div { class: "mb-3",
                Label { class: "mb-1 text-base!", html_for: "message", "Message" }
                Textarea {
                    id: "message",
                    name: "message",
                    variant: TextareaVariant::default(),
                    required: true,
                    rows: "5",
                    placeholder: "Enter your message",
                }
            }
            Button { r#type: "submit", class: "max-sm:w-full max-sm:mb-8", "Send Message" }
            AlertDialog { open: open(), on_open_change: move |v| open.set(v),
                AlertDialogTitle { "Subscription Ended" }
                AlertDialogDescription {
                    "This form doesn't work anymore cause my subscription ended lol!
                    You can contact me thru my social medias instead."
                }
                AlertDialogActions {
                    AlertDialogCancel { "I'll donate!" }
                    AlertDialogAction { "Broke ass nigga!" }
                }
            }
        }
    }
}