use yew::prelude::*;

use crate::components::{
    buttons::Button,
    inputs::{
        Email,
        Password,
        Submit,
        Kind,
        Input,
    },
    layout::Alignment::{
        Left,
        Center,
        Right, 
    },
};

#[function_component(SinkPage)]
pub fn sink_page() -> Html {
    html!{
        <>
            <h1>{"Component Sink"}</h1>

            <h2>{"Buttons"}</h2>
            <Button/>

            <h2>{"Input Fields"}</h2>
            <p>{"Email"}</p>
            <Email alignment={Center}/> 
            <Password alignment={Left} />
            <Password />
            <Submit />
            <Input kind={Kind::Email} />
            <Input kind={Kind::Text} />

            <h2>{"Forms"}</h2>
            <p>{"Form is a convenience wrapper around related input fields"}</p>
            <p>{"This is done to make the developers life much easier. The goal with the syntax is to keep the api simple, and clean so that it reads like a book"}</p>
            /*<Form container_alignment={Center} input_alignment={Center} />*/
        </>        
    }
}