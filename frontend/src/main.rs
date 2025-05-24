use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "JUtils ERP | Rust Frontend" }</h1>
            <CustomerForm />
        </>
    }
}

fn main() {
    std::println!("Hello World!");
    yew::start_app::<App>();
}