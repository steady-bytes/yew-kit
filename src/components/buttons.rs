use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Button)]
pub fn button() -> Html {
    html!{
        <button>{"Click Me"}</button>
    }
}

#[function_component(CartAdd)]
pub fn button() -> Html {
    html!{
        <button><Icon icon_id={IconId::FontAwesomeSolidCartPlus}/>{"Add to Cart"}</button>
    }
}

#[function_component(CartView)]
pub fn button() -> Html {
    html!{
        <button><Icon icon_id={IconId::FontAwesomeSolidCartShopping}/>{"View Cart"}</button>
    }
}