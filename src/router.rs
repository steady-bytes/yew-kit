use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    sink::SinkPage,
    index::IndexPage,
};

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Routes {
    #[at("/sink")]
    SinkPage,
    #[at("/")]
    IndexPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::SinkPage => html!{<SinkPage/>},
        Routes::IndexPage=> html!{<IndexPage/>},
        Routes::NotFound => html!{<h1>{"404"}</h1> },
    }
}