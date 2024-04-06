use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{openai_get, openai_post, openai_delete, ApiResponseOrError};
use derive_builder::Builder;
use std::collections::HashMap as Map;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Thread {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub metadata: Value,
}

#[derive(Builder, Deserialize, Serialize, Clone, Debug)]
pub struct ThreadBuilder {
    pub messages: Vec<Message>,
    pub metadata: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeletedThread {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Role {
    Owner,
    Assistant,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Owner => "owner",
            Role::Assistant => "assistant",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    /// The role of the author of this message.
    pub role: Role,
    /// The content of the message.
    pub content: String,
    /// A list of File IDs that the message should use.
    /// There can be a maximum of 10 files attached to a message.
    /// Useful for tools like retrieval and code_interpreter that can access
    /// and use files.
    pub file_ids: Option<Vec<String>>,
    /// Metadata for the message.
    #[serde(default)]
    pub metadata: Map<String, String>,
}

impl Thread {
    /// Creates a new thread.
    /// Threads are saved history that assistants can interact with.
    pub async fn create(
        messages: Vec<Message>,
        metadata: Map<String, String>,
    ) -> ApiResponseOrError<Self> {
        openai_post("threads", &serde_json::json!({ "messages": messages, "metadata": metadata })).await
    }

    /// Retrieves a thread instance,
    /// providing basic information about the thread such as the owner and metadata.
    pub async fn from(id: &str) -> ApiResponseOrError<Self> {
        openai_get(&format!("threads/{id}")).await
    }

    /// Modifies a thread instance,
    /// changing the metadata.
    pub async fn update(
        id: &str,
        metadata: Map<String, String>,
    ) -> ApiResponseOrError<Self> {
        openai_post(&format!("threads/{id}"), &serde_json::json!({ "metadata": metadata })).await
    }

    /// Deletes a thread instance.
    /// This is a permanent action and cannot be undone.
    pub async fn delete(id: &str) -> ApiResponseOrError<DeletedThread> {
        openai_delete(&format!("threads/{id}")).await
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextContent {
    text: Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    value: String,
    annotations: Vec<Annotation>, // Assuming you have a definition for Annotation
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFileContent {
    image_file: ImageFile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFile {
    file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation(FileCitationAnnotation),
    #[serde(rename = "file_path")]
    FilePath(FilePathAnnotation),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileCitationAnnotation {
    text: String,
    file_citation: FileCitation,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileCitation {
    file_id: String,
    quote: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilePathAnnotation {
    text: String,
    file_path: FilePath,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilePath {
    file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "image_file")]
    ImageFile(ImageFileContent),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IncompleteDetails {
    pub reason: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageObject {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub thread_id: String,
    pub status: String,
    #[serde(default)]
    pub incomplete_details: Option<IncompleteDetails>,
    pub role: Role,
    pub content: Content,
    pub file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: Map<String, String>
}

impl Thread {
    /// Creates a new message in the thread.
    /// Messages are saved history that assistants can interact with.
    pub async fn create_message(
        id: &str,
        role: Role,
        content: &str,
        file_ids: Option<Vec<String>>,
        metadata: Option<Value>,
    ) -> ApiResponseOrError<MessageObject> {
        openai_post(&format!("threads/{id}/messages"), &serde_json::json!({
            "role": role.as_str(),
            "content": content,
            "file_ids": file_ids,
            "metadata": metadata,
        })).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use crate::tests::DEFAULT_THREAD;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn thread() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());
        let thread = Thread::from(DEFAULT_THREAD).await.unwrap();
        assert_eq!(thread.id, DEFAULT_THREAD);
    }
}


