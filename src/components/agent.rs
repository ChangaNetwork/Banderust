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
    let request_response = use_state(|| Vec::new());
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
            let request_response = request_response.clone();

    
            move |_| {
                let request_response = request_response.clone();
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
                            // First check if the response status is OK
                            if response.ok() {
                                // Get the response body as text
                                match response.text().await {
                                    Ok(body_text) => {
                                        info!("Response body: {}", body_text);
                                        
                                        match serde_json::from_str::<AgentResponses>(&body_text) {
                                            Ok(responses) => {
                                                let mut sum_response: Vec<String> = Vec::new();
                                                info!("Parsed {} agent responses", responses.len());
                                                for response in responses {
                                                    for part in response.content.parts {
                                                        match part {
                                                            Parts::Text { text } => {
                                                                info!("{}", text);
                                                                sum_response.push(text);
                                                            }
                                                            _ => {
                                                                info!("no match");
                                                            }
                                                        }                                                  }
                                                }
                                                request_response.set(sum_response);
                                            },
                                            
                                            Err(e) => error!("Failed to parse JSON: {}", e),
                                        }
                                    },
                                    Err(e) => error!("Failed to read response body: {}", e),
                                }
                            } else {
                                error!("Request failed with status: {}", response.status());
                            }
                        }
                        Err(err) => {
                            error!("Request error: {:?}", err);
                        }
                    }
                }); 
            }
        };

    if session.id.is_empty() {
        html! {}
    } else {
    html! {
        <>
        <input type="text" {oninput}/>
        <button {onclick}>{"Agent"}</button>
        <p>{"request text: "}<h5>{&*request_text}</h5></p>
        <h3>{ if request_response.len() > 0 {"Story:"} else {""}} </h3>
        { for request_response.iter().map(|item| html! { <p>{item}</p> }) }
        </>
        }
    }

}