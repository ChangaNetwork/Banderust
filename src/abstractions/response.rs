use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;

pub struct CreateSession {
    pub app_name: String,
    pub user_id: String,
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
#[derive(Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct CreateSessionResponse {
    pub id: String,
    pub app_name: String,
    pub user_id: String,
    //state: Option<State>,
    //events: Option<Events>,
    pub last_update_time: f64,
}


impl fmt::Display for CreateSessionResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Session(id:{}, app:{}, user:{}, last_update:{})", self.id, self.app_name, self.user_id, self.last_update_time)
    }
}

#[derive(Serialize, Deserialize, Default)]

struct State {
    #[serde(default)] 
    pub state: HashMap<String, serde_json::Value>, 
}

#[derive(Serialize, Deserialize, Default)]

struct Events {
    #[serde(default)] 
    pub events: HashMap<String, serde_json::Value>, 
}