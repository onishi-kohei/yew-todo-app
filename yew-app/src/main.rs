use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World" }</h1>
        </div>


    }
}

fn main() {
    yew::start_app::<App>();
}