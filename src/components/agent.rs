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

    let send_agent_request = {
        let session = session.clone();
        let request_response = request_response.clone();

        move |text_to_send: String| {
            let session = session.clone();
            let request_response = request_response.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let body = RunAgentBody {
                    app_name: session.app_name.clone(),
                    user_id: session.user_id.clone(),
                    session_id: session.id.clone(),
                    new_message: NewMessage {
                        role: "user".to_string(),
                        parts: vec![Parts::Text { text: text_to_send }],
                    },
                    streaming: false,
                };

                let url = "http://127.0.0.1:8000/run";
                match Request::post(url)
                    .json(&body)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.ok() {
                            match response.text().await {
                                Ok(body_text) => {
                                    match serde_json::from_str::<AgentResponses>(&body_text) {
                                        Ok(responses) => {
                                            let mut sum_response = Vec::new();
                                            for response in responses {
                                                for part in response.content.parts {
                                                    if let Parts::Text { text } = part {
                                                        sum_response.push(text);
                                                    }
                                                }
                                            }
                                            request_response.set(sum_response);
                                        }
                                        Err(e) => error!("Failed to parse JSON: {}", e),
                                    }
                                }
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

    let oninput = {
        let value = request_text.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target().unwrap().dyn_into().unwrap();
            value.set(target.value());
        }
    };

    let on_agent_click = {
        let request_text = request_text.clone();
        let send_agent_request = send_agent_request.clone();
        move |_| {
            send_agent_request((*request_text).clone());
        }
    };

    let on_choice_click = {
        let request_response = request_response.clone(); // clone handle, not data
        let send_agent_request = send_agent_request.clone();
    
        move |index: usize| {
            let request_response = request_response.clone(); // clone again for closure
            let send_agent_request = send_agent_request.clone();
    
            Callback::from(move |_| {
                if let Some(text) = request_response.get(index) {
                    send_agent_request(text.clone());
                }
            })
        }
    };
    

    if session.id.is_empty() {
        html! {}
    } else {
        html! {
            <>
            <input type="text" {oninput}/>
            <button onclick={on_agent_click}>{"Agent"}</button>
            <p>{"request text: "}<h5>{&*request_text}</h5></p>
            <h3>{ if request_response.len() > 0 { "Story:" } else { "" }} </h3>
            {
                if request_response.len() > 0 {
                    html! {
                        <>
                        <p>{&request_response[0]}</p>
                        <button onclick={on_choice_click(1)}>{"Choice A"}</button>
                        <p>{&request_response[1]}</p>
                        <button onclick={on_choice_click(2)}>{"Choice B"}</button>
                        <p>{&request_response[2]}</p>
                        </>
                    }
                } else {
                    html! {}
                }
            }
            </>
        }
    }
}