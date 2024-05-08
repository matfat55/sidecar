use std::{path::PathBuf, sync::Arc};

use llm_client::{
    broker::LLMBroker,
    clients::types::LLMType,
    config::LLMBrokerConfiguration,
    provider::{AnthropicAPIKey, CodeStoryLLMTypes, LLMProvider, LLMProviderAPIKeys},
};
use sidecar::{
    agentic::{
        symbol::{events::input::SymbolInputEvent, manager::SymbolManager},
        tool::{
            broker::ToolBroker,
            code_edit::models::broker::CodeEditBroker,
            code_symbol::important::{
                CodeSymbolImportantResponse, CodeSymbolWithSteps, CodeSymbolWithThinking,
            },
        },
    },
    chunking::{editor_parsing::EditorParsing, languages::TSLanguageParsing},
    inline_completion::symbols_tracker::SymbolTrackerInline,
    user_context::types::UserContext,
};

fn default_index_dir() -> PathBuf {
    match directories::ProjectDirs::from("ai", "codestory", "sidecar") {
        Some(dirs) => dirs.data_dir().to_owned(),
        None => "codestory_sidecar".into(),
    }
}

// TODO: we need more symbol intelligence somehow to be able to do things

#[tokio::main]
async fn main() {
    let current_query = "I want to create a new groq provider".to_owned();
    let anthropic_api_key = "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".to_owned();
    let api_key = LLMProviderAPIKeys::Anthropic(AnthropicAPIKey::new(anthropic_api_key));
    let user_context = UserContext::new(
        vec![],
        vec![],
        None,
        vec!["/Users/skcd/scratch/sidecar/llm_client".to_owned()],
    );
    // this is the current running debuggable editor
    let editor_url = "http://localhost:64276".to_owned();
    let editor_parsing = Arc::new(EditorParsing::default());
    let symbol_broker = Arc::new(SymbolTrackerInline::new(editor_parsing.clone()));
    let tool_broker = Arc::new(ToolBroker::new(
        Arc::new(
            LLMBroker::new(LLMBrokerConfiguration::new(default_index_dir()))
                .await
                .expect("to initialize properly"),
        ),
        Arc::new(CodeEditBroker::new()),
        symbol_broker.clone(),
        Arc::new(TSLanguageParsing::init()),
    ));
    let symbol_manager = SymbolManager::new(
        tool_broker.clone(),
        symbol_broker.clone(),
        editor_url.to_owned(),
    );
    let symbol_input = SymbolInputEvent::new(
        user_context,
        LLMType::ClaudeHaiku,
        LLMProvider::Anthropic,
        api_key,
        current_query.to_owned(),
    );

    // execute input on manager
    let _ = symbol_manager.initial_request(symbol_input).await;

    // after the initial request this is the reply we get back, so lets try to make this work end to end for this case

    let request = CodeSymbolImportantResponse::new(
        vec![
            CodeSymbolWithThinking::new("LLMProvider".to_owned(), "We need to add a new variant to the LLMProvider enum to support the new Groq provider.".to_owned(), "/Users/skcd/scratch/sidecar/llm_client/src/provider.rs".to_owned()),
            CodeSymbolWithThinking::new("LLMProviderAPIKeys".to_owned(),"We need to add a new variant to the LLMProviderAPIKeys enum to hold the API key for the Groq provider.".to_owned(), "/Users/skcd/scratch/sidecar/llm_client/src/provider.rs".to_owned()),
            CodeSymbolWithThinking::new("LLMBroker".to_owned(),"We need to add support for the new Groq provider in the LLMBroker struct and its methods.".to_owned(), "/Users/skcd/scratch/sidecar/llm_client/src/broker.rs".to_owned()),
            CodeSymbolWithThinking::new("GroqClient".to_owned(),"We need to create a new GroqClient struct that implements the LLMClient trait to handle requests for the Groq provider.".to_owned(),"/Users/skcd/scratch/sidecar/llm_client/src/clients/groq.rs".to_owned())],
        vec![
            CodeSymbolWithSteps::new("LLMProvider".to_owned(),vec!["Add a new variant to the LLMProvider enum for the Groq provider:\n\n```rust\n#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]\npub enum LLMProvider {\n    // ...\n    Groq,\n}\n```".to_owned()],false,"/Users/skcd/scratch/sidecar/llm_client/src/provider.rs".to_owned()), 
            CodeSymbolWithSteps::new("LLMProviderAPIKeys".to_owned(), vec!["Add a new variant to the LLMProviderAPIKeys enum to hold the API key for the Groq provider:\n\n```rust\n#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]\npub struct GroqAPIKey {\n    pub api_key: String,\n}\n\n#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]\npub enum LLMProviderAPIKeys {\n    // ...\n    Groq(GroqAPIKey),\n}\n```".to_owned()],false,"/Users/skcd/scratch/sidecar/llm_client/src/provider.rs".to_owned()),
            CodeSymbolWithSteps::new("LLMBroker".to_owned(), vec!["1. In the `LLMBroker::new` method, add the new Groq provider:\n\n```rust\npub async fn new(config: LLMBrokerConfiguration) -> Result<Self, LLMClientError> {\n    // ...\n    Ok(broker\n        // ...\n        .add_provider(LLMProvider::Groq, Box::new(GroqClient::new())))\n}\n```\n\n2. In the `LLMBroker::get_provider` method, add a case for the Groq provider:\n\n```rust\nfn get_provider(&self, api_key: &LLMProviderAPIKeys) -> LLMProvider {\n    match api_key {\n        // ...\n        LLMProviderAPIKeys::Groq(_) => LLMProvider::Groq,\n    }\n}\n```\n\n3. In the `LLMBroker::stream_completion` and `LLMBroker::stream_string_completion` methods, add a case for the Groq provider:\n\n```rust\nlet provider_type = match &api_key {\n    // ...\n    LLMProviderAPIKeys::Groq(_) => LLMProvider::Groq,\n};\n```".to_owned()],false,"/Users/skcd/scratch/sidecar/llm_client/src/broker.rs".to_owned()),
            CodeSymbolWithSteps::new("GroqClient".to_owned(), vec!["Create a new file `groq.rs` in the `clients` directory and implement the `GroqClient` struct and the `LLMClient` trait:\n\n```rust\nuse async_trait::async_trait;\nuse tokio::sync::mpsc::UnboundedSender;\n\nuse crate::provider::LLMProviderAPIKeys;\n\nuse super::types::{\n    LLMClient, LLMClientCompletionRequest, LLMClientCompletionResponse,\n    LLMClientCompletionStringRequest, LLMClientError,\n};\n\npub struct GroqClient {\n    // Add any necessary fields for the Groq client\n}\n\nimpl GroqClient {\n    pub fn new() -> Self {\n        // Initialize the Groq client\n        Self { /* ... */ }\n    }\n\n    // Add any other necessary methods for the Groq client\n}\n\n#[async_trait]\nimpl LLMClient for GroqClient {\n    fn client(&self) -> &crate::provider::LLMProvider {\n        &crate::provider::LLMProvider::Groq\n    }\n\n    async fn stream_completion(\n        &self,\n        api_key: LLMProviderAPIKeys,\n        request: LLMClientCompletionRequest,\n        sender: UnboundedSender<LLMClientCompletionResponse>,\n    ) -> Result<String, LLMClientError> {\n        // Implement the stream_completion method for the Groq client\n        todo!()\n    }\n\n    async fn completion(\n        &self,\n        api_key: LLMProviderAPIKeys,\n        request: LLMClientCompletionRequest,\n    ) -> Result<String, LLMClientError> {\n        // Implement the completion method for the Groq client\n        todo!()\n    }\n\n    async fn stream_prompt_completion(\n        &self,\n        api_key: LLMProviderAPIKeys,\n        request: LLMClientCompletionStringRequest,\n        sender: UnboundedSender<LLMClientCompletionResponse>,\n    ) -> Result<String, LLMClientError> {\n        // Implement the stream_prompt_completion method for the Groq client\n        todo!()\n    }\n}\n```".to_owned()],true,"/Users/skcd/scratch/sidecar/llm_client/src/clients/groq.rs".to_owned())
        ]
    );

    // Lets execute the first event
    // let response = mecha.iterate(MechaEvent::InitialRequest(mecha_input)).await;
    // let important_symbols = mecha.important_symbols(request).await;
    // println!("{:?}", important_symbols);

    // next we can start multiple mechas based on this context
    // let new_mechas: Vec<_> = important_symbols
    //     .expect("to work")
    //     .into_iter()
    //     .map(|symbol| {
    //         MechaBasic::with_symbol(
    //             symbol,
    //             symbol_broker.clone(),
    //             tool_broker.clone(),
    //             Some(current_query.to_owned()),
    //             editor_parsing.clone(),
    //             symbol_locking.clone(),
    //             editor_url.to_owned(),
    //         )
    //     })
    //     .collect();

    // for simplicity we will focus on just the first one and see how to get that
    // working and reporting that back to the editor
    // println!("hello world");
}
