use yew::prelude::*;
use gloo_net::http::Request;
use log::{info, error};

use crate::abstractions::response::{CreateSession, CreateSessionResponse};




#[derive(Clone, PartialEq, Properties)]
pub struct SessionProp {
    pub session: CreateSessionResponse
}
#[function_component]
pub fn SessionInformation(props: &SessionProp) -> Html {
    let session: CreateSessionResponse = props.session.clone();
    if session.id.is_empty() {
        html! {}
    } else {
        html! {
            <>
                <h2>{"Session information:"}</h2>
                <p>{"id: "}{session.id}</p>
                <p>{"user id: "}{session.user_id}</p>
                <p>{"app name: "}{session.app_name}</p>
            </>
        } 
    } 
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSessionProps {
    pub counter: bool,
    pub session: CreateSessionResponse,
    pub on_counter_change: Callback<bool>,
    pub on_session_change: Callback<CreateSessionResponse>,
}


#[function_component]
pub fn OpenSession(props: &CreateSessionProps) -> Html {
    let counter = props.counter;
    let session = props.session.clone();
    let on_counter_change = props.on_counter_change.clone();
    let on_session_change = props.on_session_change.clone();

    let onclick = {
        let on_counter_change = on_counter_change.clone();
        let on_session_change = on_session_change.clone();
        let current_counter = counter.clone();
        let create_session = CreateSession{
            app_name: "multi_tool_agent".to_string(),
            user_id: "user_1".to_string(),
        };
    
        move |_| {
        let on_counter_change = on_counter_change.clone();
        let on_session_change = on_session_change.clone();
        if !current_counter {
            let url = format!("http://127.0.0.1:8000/apps/{}/users/{}/sessions", create_session.app_name, create_session.user_id);
            wasm_bindgen_futures::spawn_local(async move {
                let response: CreateSessionResponse = Request::post(&url)
                    //.body() 
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    info!("Created Session {}", response);
                    on_session_change.emit(response);
                    //on_counter_change.emit(true);
                    });
            } else {
                let url = format!("http://127.0.0.1:8000/apps/{}/users/{}/sessions/{}", session.app_name, session.user_id, session.id);
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::delete(&url)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                on_session_change.emit(CreateSessionResponse::default());
                                info!("Session terminated.");
                            } else {
                                error!("Delete failed: {}", response.status());
                            }
                        }
                        Err(e) => error!("Request failed: {}", e),
                    }
                });
            }
            on_counter_change.emit(!current_counter);
            //counter.set(if *counter {false} else {true});
        }
    };

html! {
        <>
        <h2>{ if counter { "Sessione attiva" }  else { "Sessione inattiva" } }</h2>
        <button {onclick}>{ if counter {"Chiudi Sessione "} else { "Crea Sessione "} }</button>
        </>
    }
}


