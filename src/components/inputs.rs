use yew::prelude::*;

use crate::components::{
    buttons::Button,
    layout::Alignment,
};

#[derive(PartialEq, Properties)]
pub struct LabelProps {
   pub for_: &'static str,
   pub text: &'static str, 
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html!{
        <label for={props.for_.clone()} text={props.text.clone()}/>
    }
}


#[derive(PartialEq)]
pub enum Kind {
    Text,
    Email,
}

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub kind: Kind, 
    pub label: Option<LabelProps>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    /// find a way to check label Option, then pass in the label component

    html!{
        if props.kind == Kind::Text {
            <>
                <Label for_="value" text="This is the text input field" />
                <input type="text" />
            </>
        } else if props.kind == Kind::Email {
            <>
                <label />
                <input type="email" />
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct EmailProps {
    /// Alignment will override what is set in the `Form`.
    pub alignment: Alignment,
}

#[function_component(Email)]
pub fn email(props: &EmailProps) -> Html {
    html!{
        <input type="email"/>
    }
}

#[derive(PartialEq, Properties)]
pub struct PasswordProps {
    pub alignment: Option<Alignment>,
}

#[function_component(Password)]
pub fn password(props: &PasswordProps) -> Html {
    html!{
        <input type="password"/>
    }
}

#[function_component(Submit)]
pub fn submit() -> Html {
    html!{
        <Button />
    }
}