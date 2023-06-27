use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub button_text: Option<String>,
    pub button_icon: Option<String>,
}

#[function_component(DynamicTextBtn)]
pub fn button(props: &Props) -> Html {
    let button_text = props.button_text.clone().unwrap_or_else(|| "Default Text".to_string());
    html! {
        <button>{button_text.as_str()}</button>
    }
}

#[function_component(DynamicIconBtn)]
pub fn button(props: &Props) -> Html {
    let button_icon = props.button_icon.clone().unwrap_or_else(|| "fa-solid fa-dragon".to_string());
    html! {
        <button><i class={button_icon.as_str()}></i></button> // Borrowed value does not live long enough???
    }
}


#[function_component(Button)]
pub fn button() -> Html {
    html! {
        <button>{"Click Me"}</button>
    }
}

#[function_component(CartAdd)]
pub fn button() -> Html {
    html! {
        <button><i class="fa-solid fa-cart-plus"/>{"Add to Cart"}</button>
    }
}

#[function_component(CartView)]
pub fn button() -> Html {
    html! {
        <button><i class="fa-solid fa-cart-shopping"/>{"View Cart"}</button>
    }
}

#[function_component(Cart)]
pub fn button() -> Html {
    html! {
        <button><i class="fa-solid fa-cart-plus"/></button>
    }
}


