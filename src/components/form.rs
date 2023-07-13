use yew::prelude::*;

use crate::components::layout::Alignment;

#[derive(PartialEq, Properties)]
pub struct FormProps {
    pub children: Children,
    pub container_alignment: Alignment,
    pub input_alignment: Alignment,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    html!{
        <form id="form">
            { for props.children.clone() }
        </form>
    }
}