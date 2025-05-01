use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)] // This helps with deserializing the different variants
pub enum Parts {
    Text { text: String },
    Thought { thought: bool },
    ExecutableCode {
        code: String,
        language: CodeLanguage,
    },
    CodeExecutionResult {
        outcome: Outcome,
        output: String,
    },
    FileData {
        file_uri: String,
        mime_type: String,
    },
    InlineData {
        data: String,
        mime_type: String,
    },
    FunctionCall {
        id: String,
        name: String,
        args: serde_json::Value,
    },
    FunctionResponse {
        id: String,
        name: String,
        response: serde_json::Value,
    },
    VideoMetadata {
        start_offset: String,
        end_offset: String,
    },
}

impl fmt::Display for Parts {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          Parts::Text { text } => write!(f, "{}", text),
          _ => write!(f, ""),
      }
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeLanguage {
    #[serde(rename = "LANGUAGE_UNSPECIFIED")]
    LanguageUnspecified,
    Python,
}

/*  #0"OUTCOME_UNSPECIFIED"
    #1"OUTCOME_OK"
    #2"OUTCOME_FAILED"
    #3"OUTCOME_DEADLINE_EXCEEDED" 
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    #[serde(rename = "OUTCOME_UNSPECIFIED")]
    OutcomeUnspecified,
    #[serde(rename = "OUTCOME_OK")]
    OutcomeOk,
    #[serde(rename = "OUTCOME_FAILED")]
    OutcomeFailed,
    #[serde(rename = "OUTCOME_DEADLINE_EXCEEDED")]
    OutcomeDeadlineExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub parts: Vec<Parts>, // Note: Changed from Parts to Vec<Parts> based on JSON
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAgentBody {
    pub app_name: String,
    pub user_id: String,
    pub session_id: String,
    pub new_message: NewMessage,
    pub streaming: bool,
}
/*
{
  "app_name": "string",
  "user_id": "string",
  "session_id": "string",
  "new_message": {
    "parts": [
      {
        "videoMetadata": {
          "endOffset": "string",
          "startOffset": "string"
        },
        "thought": true,
        "codeExecutionResult": {
          "outcome": "OUTCOME_UNSPECIFIED",
          "output": "string"
        },
        "executableCode": {
          "code": "string",
          "language": "LANGUAGE_UNSPECIFIED"
        },
        "fileData": {
          "fileUri": "string",
          "mimeType": "string"
        },
        "functionCall": {
          "id": "string",
          "args": {
            "additionalProp1": {}
          },
          "name": "string"
        },
        "functionResponse": {
          "id": "string",
          "name": "string",
          "response": {
            "additionalProp1": {}
          }
        },
        "inlineData": {
          "data": "string",
          "mimeType": "string"
        },
        "text": "string"
      }
    ],
    "role": "string"
  },
  "streaming": false
}
*/


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedContext {
    pub text: String,
    pub title: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebContext {
    pub domain: String,
    pub title: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_context: Option<RetrievedContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub end_index: i32,
    pub part_index: i32,
    pub start_index: i32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingSupport {
    pub confidence_scores: Vec<f32>,
    pub grounding_chunk_indices: Vec<i32>,
    pub segment: Segment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search_dynamic_retrieval_score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntryPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_blob: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingMetadata {
    pub grounding_chunks: Vec<GroundingChunk>,
    pub grounding_supports: Vec<GroundingSupport>,
    pub retrieval_metadata: RetrievalMetadata,
    pub retrieval_queries: Vec<String>,
    pub search_entry_point: SearchEntryPoint,
    pub web_search_queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDelta {
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactDelta {
    #[serde(flatten)]
    pub additional_props: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthScheme {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<HttpCredentials>,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccountCredential {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
    pub universe_domain: String,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccountAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_credential: Option<ServiceAccountCredential>,
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_credential: Option<bool>,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2 {
    pub client_id: String,
    pub client_secret: String,
    pub auth_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_response_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAuthCredential {
    pub auth_type: String,
    pub resource_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ServiceAccountAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<OAuth2>,
    #[serde(flatten)]
    pub additional_props: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_scheme: AuthScheme,
    pub raw_auth_credential: RawAuthCredential,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchanged_auth_credential: Option<RawAuthCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedAuthConfigs {
    #[serde(flatten)]
    pub configs: HashMap<String, AuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_summarization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_delta: Option<StateDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_delta: Option<ArtifactDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_auth_configs: Option<RequestedAuthConfigs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub content: NewMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_metadata: Option<GroundingMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_running_tool_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

pub type AgentResponses = Vec<AgentResponse>;

/*
[
  {
    "content": {
      "parts": [
        {
          "videoMetadata": {
            "endOffset": "string",
            "startOffset": "string"
          },
          "thought": true,
          "codeExecutionResult": {
            "outcome": "OUTCOME_UNSPECIFIED",
            "output": "string"
          },
          "executableCode": {
            "code": "string",
            "language": "LANGUAGE_UNSPECIFIED"
          },
          "fileData": {
            "fileUri": "string",
            "mimeType": "string"
          },
          "functionCall": {
            "id": "string",
            "args": {
              "additionalProp1": {}
            },
            "name": "string"
          },
          "functionResponse": {
            "id": "string",
            "name": "string",
            "response": {
              "additionalProp1": {}
            }
          },
          "inlineData": {
            "data": "string",
            "mimeType": "string"
          },
          "text": "string"
        }
      ],
      "role": "string"
    },
    "grounding_metadata": {
      "groundingChunks": [
        {
          "retrievedContext": {
            "text": "string",
            "title": "string",
            "uri": "string"
          },
          "web": {
            "domain": "string",
            "title": "string",
            "uri": "string"
          }
        }
      ],
      "groundingSupports": [
        {
          "confidenceScores": [
            0
          ],
          "groundingChunkIndices": [
            0
          ],
          "segment": {
            "endIndex": 0,
            "partIndex": 0,
            "startIndex": 0,
            "text": "string"
          }
        }
      ],
      "retrievalMetadata": {
        "googleSearchDynamicRetrievalScore": 0
      },
      "retrievalQueries": [
        "string"
      ],
      "searchEntryPoint": {
        "renderedContent": "string",
        "sdkBlob": "string"
      },
      "webSearchQueries": [
        "string"
      ]
    },
    "partial": true,
    "turn_complete": true,
    "error_code": "string",
    "error_message": "string",
    "interrupted": true,
    "custom_metadata": {
      "additionalProp1": {}
    },
    "invocation_id": "",
    "author": "string",
    "actions": {
      "skip_summarization": true,
      "state_delta": {
        "additionalProp1": {}
      },
      "artifact_delta": {
        "additionalProp1": 0,
        "additionalProp2": 0,
        "additionalProp3": 0
      },
      "transfer_to_agent": "string",
      "escalate": true,
      "requested_auth_configs": {
        "additionalProp1": {
          "auth_scheme": {
            "type": "apiKey",
            "description": "string",
            "in": "query",
            "name": "string",
            "additionalProp1": {}
          },
          "raw_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          },
          "exchanged_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          }
        },
        "additionalProp2": {
          "auth_scheme": {
            "type": "apiKey",
            "description": "string",
            "in": "query",
            "name": "string",
            "additionalProp1": {}
          },
          "raw_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          },
          "exchanged_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          }
        },
        "additionalProp3": {
          "auth_scheme": {
            "type": "apiKey",
            "description": "string",
            "in": "query",
            "name": "string",
            "additionalProp1": {}
          },
          "raw_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          },
          "exchanged_auth_credential": {
            "auth_type": "apiKey",
            "resource_ref": "string",
            "api_key": "string",
            "http": {
              "scheme": "string",
              "credentials": {
                "username": "string",
                "password": "string",
                "token": "string",
                "additionalProp1": {}
              },
              "additionalProp1": {}
            },
            "service_account": {
              "service_account_credential": {
                "type": "",
                "project_id": "string",
                "private_key_id": "string",
                "private_key": "string",
                "client_email": "string",
                "client_id": "string",
                "auth_uri": "string",
                "token_uri": "string",
                "auth_provider_x509_cert_url": "string",
                "client_x509_cert_url": "string",
                "universe_domain": "string",
                "additionalProp1": {}
              },
              "scopes": [
                "string"
              ],
              "use_default_credential": false,
              "additionalProp1": {}
            },
            "oauth2": {
              "client_id": "string",
              "client_secret": "string",
              "auth_uri": "string",
              "state": "string",
              "redirect_uri": "string",
              "auth_response_uri": "string",
              "auth_code": "string",
              "access_token": "string",
              "refresh_token": "string",
              "additionalProp1": {}
            },
            "additionalProp1": {}
          }
        }
      }
    },
    "long_running_tool_ids": [
      "string"
    ],
    "branch": "string",
    "id": "",
    "timestamp": 0
  }
]
*/