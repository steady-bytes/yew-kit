use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                    <Switch<router::Routes> render={router::switch} />
            </BrowserRouter>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}