use yew::prelude::*;
use components::header::Header;

mod components;
#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Header />
            <h1>{ "Hello World" }</h1>
        </div>


    }
}

fn main() {
    yew::start_app::<App>();
}