use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;

mod components;
#[function_component(App)]
fn app() -> Html {

    let on_add = {
        Callback::from(move |title: String| {
            log::info!("on_add: {:?}", title);
        })
    };

    html! {

        <>
            <Header />
            <main class="container-fluid mt-2">
                <TodoForm {on_add} />
                <TodoList />
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}