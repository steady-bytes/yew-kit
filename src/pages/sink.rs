use yew::prelude::*;

use crate::components::buttons::Button;

#[function_component(SinkPage)]
pub fn sink_page() -> Html {
    html!{
        <>
            <h1>{"Component Sink"}</h1>

            <h2>{"Buttons"}</h2>
            <Button/>
        </>        
    }
}