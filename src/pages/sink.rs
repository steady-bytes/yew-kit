use yew::prelude::*;

use crate::components::buttons::{Button, CartAdd, CartView, Cart, DynamicTextBtn, DynamicIconBtn};

#[function_component(SinkPage)]
pub fn sink_page() -> Html {
    html!{
        <>
            <h1>{"Component Sink"}</h1>

            <h2>{"Buttons"}</h2>
                <div>
                    <Button/>
                </div>
            <h3>{"Shopping Cart"}</h3>
                <div>
                    <span><CartAdd/></span>
                    <span><CartView/></span>
                    <span><Cart/></span>
                </div>
            <h3>{"Dynamic Icons/Text from props"}</h3>
                <div>
                    <span><DynamicTextBtn button_text="Clickety Click"/></span>
                    <span><DynamicIconBtn button_icon="fa-solid fa-cart"/></span>
                </div>
        </>        
    }
}