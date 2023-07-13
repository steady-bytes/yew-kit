use std::collections::HashMap;
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

#[derive(PartialEq, Copy, Clone)]
pub enum InputType{
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
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
    pub value: Option<&'static str>,
    pub name: Option<&'static str>,
    pub id: Option<&'static str>,
}

pub fn make_input_map() -> HashMap<String, InputProps> {
    let mut input_map: HashMap<String, InputProps> = HashMap::new();

    input_map.insert(String::from("button"), InputProps{kind: InputType::Button, value: Some("button"), name: Some("Button"), id: Some("button_id")});
    input_map.insert(String::from("checkbox"), InputProps { kind: InputType::Checkbox, value: Some("checkbox"), name: Some("Checkbox"), id: Some("checkbox")});
    input_map.insert(String::from("color"), InputProps{kind: InputType::Color, value: Some("color"), name: Some("Color"), id: Some("color")});
    input_map.insert(String::from("date"), InputProps{kind: InputType::Date, value: Some("date"), name: Some("Date"), id: Some("date")});
    input_map.insert(String::from("datetimeLocal"), InputProps{kind: InputType::DatetimeLocal, value: Some("datetimeLocal"), name: Some("DatetimeLocal"), id: Some("datetimeLocal")});
    input_map.insert(String::from("email"), InputProps{kind: InputType::Email, value: Some("email"), name: Some("Email"), id: Some("email")});
    input_map.insert(String::from("file"), InputProps{ kind: InputType::File, value: None, name: Some("File"), id: Some("file")});
    input_map.insert(String::from("hidden"), InputProps{kind: InputType::Hidden, value: Some("hidden"), name: Some("Hidden"), id: Some("hidden")});
    input_map.insert(String::from("month"), InputProps{kind: InputType::Month, value: Some("month"), name: Some("Month"), id: Some("month")});
    input_map.insert(String::from("number"), InputProps{kind: InputType::Number, value: Some("number"), name: Some("Number"), id: Some("number")});
    input_map.insert(String::from("password"), InputProps{kind: InputType::Password, value: Some("password"), name: Some("Password"), id: Some("password")});
    input_map.insert(String::from("radio"), InputProps{kind: InputType::Radio, value: Some("radio"), name: Some("Radio"), id: Some("radio")});
    input_map.insert(String::from("range"), InputProps{kind: InputType::Range, value: Some("range"), name: Some("Range"), id: Some("range")});
    input_map.insert(String::from("reset"), InputProps{kind: InputType::Reset, value: Some("reset"), name: Some("Reset"), id: Some("reset")});
    input_map.insert(String::from("search"), InputProps{kind: InputType::Search, value: Some("search"), name: Some("Search"), id: Some("search")});
    input_map.insert(String::from("submit"), InputProps{kind: InputType::Submit, value: Some("submit"), name: Some("Submit"), id: Some("submit")});
    input_map.insert(String::from("tel"), InputProps{kind: InputType::Tel, value: Some("tel"), name: Some("Tel"), id: Some("tel")});
    input_map.insert(String::from("text"), InputProps{kind: InputType::Text, value: Some("text"), name: Some("Text"), id: Some("text")});
    input_map.insert(String::from("time"), InputProps{kind: InputType::Time, value: Some("time"), name: Some("Time"), id: Some("time")});
    input_map.insert(String::from("url"), InputProps{kind: InputType::Url, value: Some("url"), name: Some("Url"), id: Some("url")});
    input_map.insert(String::from("week"), InputProps{kind: InputType::Week, value: Some("week"), name: Some("Week"), id: Some("week")});

    input_map
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input 
            type={props.kind.to_string()} 
            value={props.value.clone()} 
            id={props.value.clone()}
        />     
    }
}