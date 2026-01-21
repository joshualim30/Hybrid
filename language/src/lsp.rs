use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use hybrid::lexer::Lexer;
use hybrid::parser::Parser;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Mutex<HashMap<String, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Hybrid language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        self.documents.lock().await.insert(uri.clone(), text.clone());
        self.on_change(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = &params.content_changes[0].text;
        self.documents.lock().await.insert(uri.clone(), text.clone());
        self.on_change(&uri, text).await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let keywords = vec![
            "var", "const", "block", "return", "if", "else", "while", "speak",
            "int", "float", "string", "bool", "void", "null", "array", "map",
            "true", "false",
        ];

        let items = keywords
            .into_iter()
            .map(|k| CompletionItem {
                label: k.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }
}

impl Backend {
    async fn on_change(&self, uri: &str, text: &str) {
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        
        let mut diagnostics = Vec::new();
        
        if let Err(e) = parser.parse() {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: e.line as u32,
                        character: e.column as u32,
                    },
                    end: Position {
                        line: e.line as u32,
                        character: (e.column + 5) as u32, // Highlight a few chars
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: e.message,
                ..Default::default()
            });
        }
        
        self.client
            .publish_diagnostics(
                Url::parse(uri).unwrap(),
                diagnostics,
                None,
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
