use std::fmt;
use yew::prelude::*;


/// LABEL

#[derive(PartialEq, Properties)]
pub struct LabelProps {
   pub text: &'static str, 
   pub html_for: Option<&'static str>,
   pub br: Option<bool>, 
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html!{
        <>
            <label for={props.html_for.clone()}>{props.text.clone()}</label>
            if props.br.unwrap_or_default() {
                <br/>
            }
        </>
    }
}

/// INPUT
/// REF: https://www.w3schools.com/html/html_form_input_types.asp

#[derive(PartialEq)]
pub enum InputType{
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            InputType::Button => "button",
            InputType::Checkbox => "checkbox",
            InputType::Color => "color",
            InputType::Date => "date",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Email => "email",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Image => "image",
            InputType::Month => "month",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            InputType::Range => "range",
            InputType::Reset => "reset",
            InputType::Search => "search",
            InputType::Submit => "submit",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Time => "time",
            InputType::Url => "url",
            InputType::Week => "week",
        };
        
        write!(f, "{}", variant_str)
    }
}

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub kind: InputType, 
    pub value: Option<&'static str>
}

// What is a smart way to map enums to string

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input type={props.kind.to_string()} value={props.value.clone()}/>
    }
}