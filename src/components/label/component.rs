use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/label/style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    let base = attributes!(label {
        class: Styles::dx_label,
    });
    let merged = merge_attributes(vec![base, props.attributes]);    

    rsx! {
        label::Label {
            // class: Styles::dx_label,
            html_for: props.html_for,
            attributes: merged,
            {props.children}
        }
    }
}
