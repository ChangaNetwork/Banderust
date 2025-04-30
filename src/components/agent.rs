use yew::prelude::*;
use gloo_net::http::Request;
use log::{info, error};
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;

use crate::abstractions::response::CreateSessionResponse;
use crate::components::session::SessionProp;
use crate::abstractions::agent::{RunAgentBody, AgentResponses, Parts, NewMessage};


#[function_component]
pub fn SendRequest(props: &SessionProp) -> Html {
    let session: CreateSessionResponse = props.session.clone();
    let request_text = use_state(|| String::new());
    let oninput = Callback::from({
        let value = request_text.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap()
                .dyn_into()
                .unwrap();
            //web_sys::console::log_1(&target.value().into()); // <- can console the value.
            value.set(target.value());
        }
    });
    let onclick = {
            let session = session.clone();
            let request_text = request_text.clone();
    
            move |_| {
                let body = RunAgentBody {
                    app_name: session.app_name.clone(),
                    user_id: session.user_id.clone(),
                    session_id: session.id.clone(),
                    new_message: NewMessage {
                        role: "user".to_string(),
                        parts: vec![
                            Parts::Text {
                                text: request_text.to_string(),
                            },
                        ],
                    },
                    streaming: false,
                };
    
                let url = "http://127.0.0.1:8000/run";
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::post(url)
                        .json(&body)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                info!("Agent request sent successfully");
                            } else {
                                error!("Failed to send request: {}", response.status());
                            }
                        }
                        Err(err) => {
                            error!("Request error: {:?}", err);
                        }
                    }
                });
            }
        };

    html! {
        <>
        <input type="text" {oninput}/>
        <button {onclick}>{"Agent"}</button>
        <p>{"request text: "}<h5>{&*request_text}</h5></p>
        </>
    }

}