use yew::prelude::*;
use gloo_net::http::Request;
use log::info;
//use wasm_bindgen_futures::wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;


struct CreateSession {
    app_name: String,
    user_id: String,
}

/* Session response:
{
  "id": "string",
  "app_name": "string",
  "user_id": "string",
  "state": {
    "additionalProp1": {}
  },
  "events": [ ]
}
 */
#[derive(Serialize, Deserialize, Default, Clone)]
struct CreateSessionResponse {
    id: String,
    app_name: String,
    user_id: String,
    //state: Option<State>,
    //events: Option<Events>,
    last_update_time: f64,
}


impl fmt::Display for CreateSessionResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Session(id:{}, app:{}, user:{}, last_update:{})", self.id, self.app_name, self.user_id, self.last_update_time)
    }
}

#[derive(Serialize, Deserialize, Default)]

struct State {
    #[serde(default)] 
    state: HashMap<String, serde_json::Value>, 
}

#[derive(Serialize, Deserialize, Default)]

struct Events {
    #[serde(default)] 
    events: HashMap<String, serde_json::Value>, 
}
#[function_component]
pub fn App() -> Html {

    let counter = use_state(|| false);
    let session = use_state(|| CreateSessionResponse::default());
    //let session_holder_clone = Rc::clone(&session_holder);

    let onclick = {
        let counter = counter.clone();
        let session = session.clone();
        let create_session = CreateSession{
            app_name: "multi_agent".to_string(),
            user_id: "user_1".to_string(),
        };

        move |_| {
        let session = session.clone();
        if !*counter {
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
                    session.set(response);
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
                                session.set(CreateSessionResponse::default());  // Clear session state
                                info!("Session terminated.");
                            } else {
                                info!("Delete failed: {}", response.status());
                            }
                        }
                        Err(e) => info!("Request failed: {}", e),
                    }
                });
            }
            counter.set(if *counter {false} else {true});
        }
    };

    html! {
        <div>
            <button {onclick}>{ if *counter {"Chiudi Sessione "} else { "Crea Sessione "} }</button>
            <p>{ if *counter { "Sessione attiva" }  else { "Sessione inattiva" } }</p>
        </div>
    }
}