use yew::prelude::*;
use serde::Serialize;
use gloo_net::http::Request;

#[derive(Serialize)]
struct Customer {
    name: String,
    email: String,
    phoneNumber: String,
}

#[function_component(CustomerForm)]
fn customer_form() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let phone = use_state(|| "".to_string());

    let onsubmit = {
        let name = name.clone();
        let email = email.clone();
        let phone = phone.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // verhindert Seiten-Reload

            let name = name.clone();
            let email = email.clone();
            let phone = phone.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_customer = Customer {
                    name: (*name).clone(),
                    email: (*email).clone(),
                    phoneNumber: (*phone).clone(),
                };

                let res = Request::post("http://localhost:8080/api/customers")
                    .header("Content-Type", "application/json")
                    .json(&new_customer)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(_) => web_sys::window()
                        .unwrap()
                        .alert_with_message("Customer added!")
                        .unwrap(),
                    Err(err) => web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!("Error: {:?}", err))
                        .unwrap(),
                }
            });
        })
    };

    html! {
        <form onsubmit={onsubmit}>
            <label>{ "Name:" }<input value={(*name).clone()} oninput={Callback::from(move |e: InputEvent| name.set(event_target_value(&e))} /></label><br/>
            <label>{ "Email:" }<input value={(*email).clone()} oninput={Callback::from(move |e: InputEvent| email.set(event_target_value(&e))} /></label><br/>
            <label>{ "Telefonnummer:" }<input value={(*phone).clone()} oninput={Callback::from(move |e: InputEvent| phone.set(event_target_value(&e))} /></label><br/>
            <button type="submit">{ "Hinzufügen" }</button>
        </form>
    }
}

fn event_target_value(e: &InputEvent) -> String {
    e.target_dyn_into::<web_sys::HtmlInputElement>()
        .map(|input| input.value())
        .unwrap_or_default()
}