use yew::prelude::*;
//use wasm_bindgen_futures::wasm_bindgen::JsValue;

// componenti/struct del progetto
use crate::abstractions::response::CreateSessionResponse;
use crate::components::session::{OpenSession, SessionInformation};
use crate::components::agent::SendRequest;

#[function_component]
pub fn App() -> Html {

    let counter = use_state(|| false);
    let session = use_state(|| CreateSessionResponse::default());

    let on_counter_change = {
        let counter = counter.clone();
        Callback::from(move |new_value| {
            counter.set(new_value);
        })
    };

    let on_session_change = {
        let session = session.clone();
        Callback::from(move |new_session| {
            session.set(new_session);
        })
    };


    html! {
        <div>
        <OpenSession 
            counter={*counter} session={(*session).clone()} 
            on_counter_change={on_counter_change} on_session_change={on_session_change}
        />           
        <br/>
        <SessionInformation session={(*session).clone()} />
        <br/>
        <SendRequest session={(*session).clone()} />
        </div>
    }
}