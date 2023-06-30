use yew::prelude::*;

use crate::components::{
    buttons::Button,
    inputs::{
        InputType,
        Input,
        Label,
    },
};

#[function_component(SinkPage)]
pub fn sink_page() -> Html {
    html!{
        <>
            <h1>{"Component Sink"}</h1>

            <h2>{"Buttons"}</h2>
            <Button/>

            <h2>{"Inputs"}</h2>
            <p>{"A small mapping between html5 input types"}</p>
            <h3>{"Button"}</h3>
            <Label text={"Button Example"} br=true />
            <Input 
                kind={InputType::Button} 
                value={"Click Me"} />

            <h3>{"Checkbox"}</h3>
            <Label text={"Checkbox"} br=true />
            <Input 
                kind={InputType::Checkbox} 
                value={"Test CheckBox"} />

            <h2>{"Forms"}</h2>
            <p>{"Form is a convenience wrapper around related input fields"}</p>
            <p>{"This is done to make the developers life much easier. The goal with the syntax is to keep the api simple, and clean so that it reads like a book"}</p>
            /*<Form container_alignment={Center} input_alignment={Center} />*/
        </>        
    }
}