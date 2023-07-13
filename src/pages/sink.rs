use yew::prelude::*;

use crate::components::{
    buttons::Button,
    inputs::{
        Input,
        Label,
        make_input_map,
    },
};

#[function_component(SinkPage)]
pub fn sink_page() -> Html {
    html!{
        <>
            <h1>{"Component Sink"}</h1>

            <h2>{"Buttons"}</h2>
            <Button/>

            <InputStory />
            

            <h2>{"Forms"}</h2>
            <p>{"Form is a convenience wrapper around related input fields"}</p>
            <p>{"This is done to make the developers life much easier. The goal with the syntax is to keep the api simple, and clean so that it reads like a book"}</p>
        </>        
    }
}

#[function_component(InputStory)]
pub fn input_story() -> Html {
    let inputs_map = make_input_map();

    html! {
        <>
            <h2>{"Inputs"}</h2>
            <p>{"A small mapping between html5 input types"}</p>
            {
                inputs_map.iter().map(|(_k, v)| {
                    html!{
                        <>
                            <h3>{v.name.clone()}</h3>
                            <Label text={v.value.unwrap_or_default()} br=true />
                            <Input kind={v.kind} value={v.value.clone()} />
                        </>
                    }
                }).collect::<Html>()
            }
        </>
    }
}