use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: tokio::sync::RwLock<HashMap<String, String>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            document_map: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let uri = params.uri.to_string();
        let text = params.text;
        
        // Store document
        self.document_map.write().await.insert(uri.clone(), text.clone());

        // Run diagnostics
        self.validate_document(&params.uri, &text).await;
    }

    async fn validate_document(&self, uri: &Url, text: &str) {
        let mut diagnostics = Vec::new();

        // Use the rizz parser to find errors
        match rizz_core::parser::parse_program(text, uri.path()) {
            Ok(_ast) => {
                // No syntax errors
            }
            Err(e) => {
                // Parser or lexer error
                let error_msg = format!("{}", e);
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 0,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("rizz-lsp".to_string()),
                    message: error_msg,
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    fn get_keyword_completions() -> Vec<CompletionItem> {
        let keywords = vec![
            // Variables
            ("Ayo", "Declare mutable variable"),
            ("Yoo", "Declare constant (immutable)"),
            // Functions
            ("Bruh", "Define function"),
            ("Rizz", "Return/output statement"),
            // Control Flow
            ("Maybe", "If condition"),
            ("Unless", "Else statement"),
            ("Crazy", "For loop"),
            // Async
            ("HawkTuah", "Async function marker"),
            ("Vibe", "Spawn async task"),
            ("Chill", "Await task completion"),
            // HTTP
            ("Spit", "HTTP GET request"),
            ("Yeet", "HTTP POST request"),
            ("Flex", "HTTP PUT request"),
            ("Ghost", "HTTP DELETE request"),
            // TCP
            ("Listen", "Create TCP server"),
            ("Holla", "Connect to TCP server"),
            ("Peek", "Read from socket"),
            ("Whisper", "Write to socket"),
            ("Dip", "Close connection"),
            // File I/O
            ("Snag", "Read file"),
            ("Stash", "Write file"),
            ("KeepAdding", "Append to file"),
            ("Trash", "Delete file"),
            ("FileExists", "Check file existence"),
            // JSON
            ("Decode", "Parse JSON string"),
            ("Encode", "Convert object to JSON"),
            // Regex
            ("Hunt", "Find regex matches"),
            ("Swap", "Regex replace"),
            ("Matches", "Check if matches pattern"),
            ("Split", "Split string by regex"),
            // Error Handling
            ("Attempt", "Try block"),
            ("Eww", "Catch block"),
            ("Cringe", "Throw error/panic"),
            // JS-style aliases
            ("let", "Declare variable (JS alias)"),
            ("const", "Declare constant (JS alias)"),
            ("function", "Define function (JS alias)"),
            ("return", "Return value (JS alias)"),
            ("try", "Try block (JS alias)"),
            ("catch", "Catch block (JS alias)"),
            ("throw", "Throw error (JS alias)"),
        ];

        keywords
            .into_iter()
            .map(|(label, detail)| CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(detail.to_string()),
                insert_text: Some(label.to_string()),
                ..Default::default()
            })
            .collect()
    }

    fn get_snippet_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "function".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Define a function".to_string()),
                insert_text: Some("Bruh ${1:name}($2) {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "async-function".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Define an async function".to_string()),
                insert_text: Some("Bruh ${1:name}($2) HawkTuah {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("If statement".to_string()),
                insert_text: Some("Maybe ${1:condition} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "if-else".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("If-else statement".to_string()),
                insert_text: Some("Maybe ${1:condition} {\n\t$2\n} Unless {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "for-range".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("For loop with range".to_string()),
                insert_text: Some("Crazy ${1:i} in ${2:0}..${3:10} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "for-array".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("For loop with array".to_string()),
                insert_text: Some("Crazy ${1:item} in ${2:array} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "try-catch".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Try-catch block".to_string()),
                insert_text: Some("Attempt {\n\t$1\n} Eww (${2:error}) {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ]
    }
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
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "rizz-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "RizzScript LSP initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.on_change(TextDocumentItem {
                uri: uri.clone(),
                text: change.text,
                version: 0,
                language_id: "rizz".to_string(),
            })
            .await;
        }
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut completions = Self::get_keyword_completions();
        completions.extend(Self::get_snippet_completions());

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        let docs = self.document_map.read().await;
        if let Some(text) = docs.get(&uri) {
            // Get the word at the cursor position
            let lines: Vec<&str> = text.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let word = extract_word_at_position(line, position.character as usize);

                // Provide hover documentation for keywords
                let hover_text = get_keyword_documentation(&word);
                if let Some(doc) = hover_text {
                    return Ok(Some(Hover {
                        contents: HoverContents::Scalar(MarkedString::String(doc)),
                        range: None,
                    }));
                }
            }
        }

        Ok(None)
    }
}

fn extract_word_at_position(line: &str, col: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    if col >= chars.len() {
        return String::new();
    }

    let mut start = col;
    let mut end = col;

    // Find word boundaries
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }

    chars[start..end].iter().collect()
}

fn get_keyword_documentation(word: &str) -> Option<String> {
    match word {
        "Ayo" => Some("**Ayo** - Declare a mutable variable\n\nExample:\n```rizz\nAyo name = \"RizzScript\"\n```".to_string()),
        "Yoo" => Some("**Yoo** - Declare an immutable constant\n\nExample:\n```rizz\nYoo PI = 3.14159\n```".to_string()),
        "Bruh" => Some("**Bruh** - Define a function\n\nExample:\n```rizz\nBruh greet(name) {\n  Rizz(\"Hello, \" + name)\n}\n```".to_string()),
        "Rizz" => Some("**Rizz** - Return a value or print output\n\nExample:\n```rizz\nRizz(\"Hello World\")\n```".to_string()),
        "Maybe" => Some("**Maybe** - If conditional statement\n\nExample:\n```rizz\nMaybe age >= 18 {\n  Rizz(\"Adult\")\n}\n```".to_string()),
        "Unless" => Some("**Unless** - Else statement\n\nExample:\n```rizz\nMaybe x > 0 {\n  Rizz(\"Positive\")\n} Unless {\n  Rizz(\"Not positive\")\n}\n```".to_string()),
        "Crazy" => Some("**Crazy** - For loop\n\nExample:\n```rizz\nCrazy i in 0..5 {\n  Rizz(i)\n}\n```".to_string()),
        "HawkTuah" => Some("**HawkTuah** - Mark function as async\n\nExample:\n```rizz\nBruh fetchData() HawkTuah {\n  Ayo data = Spit(\"https://api.example.com\")\n  Rizz(data)\n}\n```".to_string()),
        "Vibe" => Some("**Vibe** - Spawn an async task\n\nExample:\n```rizz\nAyo task = Vibe fetchData()\n```".to_string()),
        "Chill" => Some("**Chill** - Await async task completion\n\nExample:\n```rizz\nAyo result = Chill(task)\n```".to_string()),
        "Spit" => Some("**Spit** - HTTP GET request\n\nExample:\n```rizz\nAyo response = Spit(\"https://api.example.com/users\")\n```".to_string()),
        "Yeet" => Some("**Yeet** - HTTP POST request\n\nExample:\n```rizz\nAyo data = {\"name\": \"Rizz\"}\nAyo response = Yeet(\"https://api.example.com/users\", data)\n```".to_string()),
        "Flex" => Some("**Flex** - HTTP PUT request\n\nExample:\n```rizz\nAyo updated = Flex(\"https://api.example.com/users/1\", data)\n```".to_string()),
        "Ghost" => Some("**Ghost** - HTTP DELETE request\n\nExample:\n```rizz\nGhost(\"https://api.example.com/users/1\")\n```".to_string()),
        "Listen" => Some("**Listen** - Create TCP/HTTP server\n\nExample:\n```rizz\nAyo server = Listen(8080, handleRequest)\n```".to_string()),
        "Holla" => Some("**Holla** - Connect to TCP server\n\nExample:\n```rizz\nAyo socket = Holla(\"localhost\", 8080)\n```".to_string()),
        "Peek" => Some("**Peek** - Read from socket\n\nExample:\n```rizz\nAyo message = Peek(socket)\n```".to_string()),
        "Whisper" => Some("**Whisper** - Write to socket\n\nExample:\n```rizz\nWhisper(socket, \"Hello\")\n```".to_string()),
        "Dip" => Some("**Dip** - Close connection\n\nExample:\n```rizz\nDip(socket)\n```".to_string()),
        "Snag" => Some("**Snag** - Read file contents\n\nExample:\n```rizz\nAyo content = Snag(\"data.txt\")\n```".to_string()),
        "Stash" => Some("**Stash** - Write to file\n\nExample:\n```rizz\nStash(\"output.txt\", \"Hello World\")\n```".to_string()),
        "KeepAdding" => Some("**KeepAdding** - Append to file\n\nExample:\n```rizz\nKeepAdding(\"log.txt\", \"New entry\\n\")\n```".to_string()),
        "Trash" => Some("**Trash** - Delete file\n\nExample:\n```rizz\nTrash(\"temp.txt\")\n```".to_string()),
        "FileExists" => Some("**FileExists** - Check if file exists\n\nExample:\n```rizz\nMaybe FileExists(\"config.rizz\") {\n  Rizz(\"Found\")\n}\n```".to_string()),
        "Decode" => Some("**Decode** - Parse JSON string (SIMD optimized)\n\nExample:\n```rizz\nAyo obj = Decode(jsonString)\n```".to_string()),
        "Encode" => Some("**Encode** - Convert object to JSON string\n\nExample:\n```rizz\nAyo json = Encode({\"key\": \"value\"})\n```".to_string()),
        "Hunt" => Some("**Hunt** - Find regex matches\n\nExample:\n```rizz\nAyo matches = Hunt(text, r\"\\d+\")\n```".to_string()),
        "Swap" => Some("**Swap** - Regex replace\n\nExample:\n```rizz\nAyo result = Swap(text, r\"\\d\", \"X\")\n```".to_string()),
        "Matches" => Some("**Matches** - Check if text matches pattern\n\nExample:\n```rizz\nMaybe Matches(text, r\"\\d{3}\") {\n  Rizz(\"Found\")\n}\n```".to_string()),
        "Split" => Some("**Split** - Split string by regex\n\nExample:\n```rizz\nAyo parts = Split(\"a,b,c\", \",\")\n```".to_string()),
        "Attempt" => Some("**Attempt** - Try block for error handling\n\nExample:\n```rizz\nAttempt {\n  Ayo data = Spit(url)\n} Eww (error) {\n  Rizz(\"Error: \" + error)\n}\n```".to_string()),
        "Eww" => Some("**Eww** - Catch block for error handling\n\nExample:\n```rizz\nAttempt {\n  riskyOperation()\n} Eww (error) {\n  Rizz(error)\n}\n```".to_string()),
        "Cringe" => Some("**Cringe** - Throw error/panic\n\nExample:\n```rizz\nCringe(\"Something went wrong!\")\n```".to_string()),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
