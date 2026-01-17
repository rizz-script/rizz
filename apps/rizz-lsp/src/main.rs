use std::collections::HashMap;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use rizz_core::lexer::{lex, Kind, Token};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: tokio::sync::RwLock<HashMap<String, String>>,
    index_map: tokio::sync::RwLock<HashMap<String, Arc<DocumentIndex>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            document_map: tokio::sync::RwLock::new(HashMap::new()),
            index_map: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let uri = params.uri.to_string();
        let text = params.text;

        // Store document
        self.document_map
            .write()
            .await
            .insert(uri.clone(), text.clone());
        // Index symbols/types
        if let Ok(idx) = DocumentIndex::build(&uri, &text) {
            self.index_map
                .write()
                .await
                .insert(uri.clone(), Arc::new(idx));
        }

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
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
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
        let uri_url = params.text_document_position_params.text_document.uri;
        let uri = uri_url.to_string();
        let position = params.text_document_position_params.position;

        let docs = self.document_map.read().await;
        if let Some(text) = docs.get(&uri) {
            // Get the word at the cursor position
            let lines: Vec<&str> = text.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let word = extract_word_at_position(line, position.character as usize);

                // Prefer symbol/type hover for identifiers
                if !word.is_empty() {
                    let idxs = self.index_map.read().await;
                    if let Some(idx) = idxs.get(&uri) {
                        if let Some(info) = idx.hover_info(&word, position) {
                            return Ok(Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: info,
                                }),
                                range: None,
                            }));
                        }
                    }

                    // Provide hover documentation for keywords/builtins
                    let hover_text = get_keyword_documentation(&word);
                    if let Some(doc) = hover_text {
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: doc,
                            }),
                            range: None,
                        }));
                    }
                }
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        let docs = self.document_map.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };
        let line = text.lines().nth(position.line as usize).unwrap_or("");
        let word = extract_word_at_position(line, position.character as usize);
        if word.is_empty() {
            return Ok(None);
        }

        // Prefer same-document resolution (closest preceding def), then any open doc
        let idxs = self.index_map.read().await;
        if let Some(idx) = idxs.get(&uri) {
            if let Some(loc) = idx.definition_location(&word, position) {
                return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
            }
        }
        for (_u, idx) in idxs.iter() {
            if let Some(loc) = idx.any_definition_location(&word) {
                return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
            }
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        let docs = self.document_map.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };
        let line = text.lines().nth(position.line as usize).unwrap_or("");
        let word = extract_word_at_position(line, position.character as usize);
        if word.is_empty() {
            return Ok(None);
        }

        let idxs = self.index_map.read().await;
        let mut out = Vec::new();
        for (u, idx) in idxs.iter() {
            out.extend(idx.find_references(u, &word));
        }
        Ok(Some(out))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        let docs = self.document_map.read().await;
        let Some(text) = docs.get(&uri) else {
            return Ok(None);
        };
        let line = text.lines().nth(position.line as usize).unwrap_or("");
        let word = extract_word_at_position(line, position.character as usize);
        if word.is_empty() {
            return Ok(None);
        }

        let idxs = self.index_map.read().await;
        let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
        for (u, idx) in idxs.iter() {
            for loc in idx.find_references(u, &word) {
                changes.entry(loc.uri.clone()).or_default().push(TextEdit {
                    range: loc.range,
                    new_text: new_name.clone(),
                });
            }
        }
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        let idxs = self.index_map.read().await;
        let Some(idx) = idxs.get(&uri) else {
            return Ok(None);
        };
        Ok(Some(DocumentSymbolResponse::Nested(idx.document_symbols())))
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

#[derive(Debug, Clone)]
struct SymbolDef {
    name: String,
    kind: SymbolKind,
    selection_range: Range,
    full_range: Range,
    detail: String, // includes type/signature
    defined_at: Position,
}

#[derive(Debug)]
struct DocumentIndex {
    uri: Url,
    defs: Vec<SymbolDef>,
    // quick lookup for “any definition”
    first_def: HashMap<String, usize>,
    // occurrences for references/rename
    occurrences: HashMap<String, Vec<Range>>,
}

impl DocumentIndex {
    fn build(uri: &str, text: &str) -> anyhow::Result<Self> {
        let uri_url = Url::parse(uri)?;
        let tokens = lex(text)?;
        let mut defs: Vec<SymbolDef> = Vec::new();
        let mut occurrences: HashMap<String, Vec<Range>> = HashMap::new();

        // record all identifier occurrences
        for t in tokens.iter() {
            if t.kind == Kind::Ident {
                let r = token_range(text, t);
                occurrences.entry(t.value.clone()).or_default().push(r);
            }
        }

        // scan for simple top-level declarations
        let mut i = 0usize;
        while i < tokens.len() {
            let k = &tokens[i].kind;
            if matches!(k, Kind::Ayo | Kind::Let | Kind::Yoo | Kind::Const) {
                // var/const decl
                if let Some(name_tok) = tokens.get(i + 1) {
                    if name_tok.kind == Kind::Ident {
                        let (ty, _next) = parse_optional_type(&tokens, i + 2);
                        let inferred = infer_initializer_type(&tokens, i + 2);
                        let ty_s = ty.or(inferred).unwrap_or_else(|| "any".to_string());
                        let kw = if matches!(k, Kind::Yoo | Kind::Const) {
                            "const"
                        } else {
                            "let"
                        };
                        let rizz_kw = if matches!(k, Kind::Yoo | Kind::Const) {
                            "Yoo"
                        } else {
                            "Ayo"
                        };
                        let kind_desc = if matches!(k, Kind::Yoo | Kind::Const) {
                            "constant"
                        } else {
                            "variable"
                        };

                        let detail = format!(
                            "```rizz\n{} {}\n```\n**Type:** `{}`\n\n*({} {})*",
                            rizz_kw, name_tok.value, ty_s, kind_desc, kw
                        );

                        defs.push(SymbolDef {
                            name: name_tok.value.clone(),
                            kind: if matches!(k, Kind::Yoo | Kind::Const) {
                                SymbolKind::CONSTANT
                            } else {
                                SymbolKind::VARIABLE
                            },
                            selection_range: token_range(text, name_tok),
                            full_range: token_range(text, name_tok),
                            detail,
                            defined_at: range_start(token_range(text, name_tok)),
                        });
                    }
                }
            } else if matches!(k, Kind::Bruh | Kind::Function) {
                if let Some(name_tok) = tokens.get(i + 1) {
                    if name_tok.kind == Kind::Ident {
                        let (sig, _end) = parse_function_signature(&tokens, i + 1);
                        defs.push(SymbolDef {
                            name: name_tok.value.clone(),
                            kind: SymbolKind::FUNCTION,
                            selection_range: token_range(text, name_tok),
                            full_range: token_range(text, name_tok),
                            detail: sig,
                            defined_at: range_start(token_range(text, name_tok)),
                        });
                    }
                }
            }
            i += 1;
        }

        let mut first_def = HashMap::new();
        for (idx, d) in defs.iter().enumerate() {
            first_def.entry(d.name.clone()).or_insert(idx);
        }

        Ok(Self {
            uri: uri_url,
            defs,
            first_def,
            occurrences,
        })
    }

    fn definition_location(&self, name: &str, pos: Position) -> Option<Location> {
        // If clicking on the definition itself, don't navigate (or could show a message)
        // Otherwise, find closest preceding definition
        let mut best: Option<&SymbolDef> = None;
        let mut best_distance = i64::MAX;

        for d in self.defs.iter().filter(|d| d.name == name) {
            // Skip if cursor is already on the definition
            if pos_in_range(pos, d.selection_range) {
                // Still return the location to allow navigation within the same file
                return Some(Location {
                    uri: self.uri.clone(),
                    range: d.selection_range,
                });
            }

            // Find closest preceding definition
            if d.defined_at.line < pos.line
                || (d.defined_at.line == pos.line && d.defined_at.character <= pos.character)
            {
                let distance = ((pos.line as i64 - d.defined_at.line as i64) * 1000)
                    + (pos.character as i64 - d.defined_at.character as i64);
                if distance < best_distance {
                    best_distance = distance;
                    best = Some(d);
                }
            }
        }
        best.map(|d| Location {
            uri: self.uri.clone(),
            range: d.selection_range,
        })
    }

    fn any_definition_location(&self, name: &str) -> Option<Location> {
        let idx = *self.first_def.get(name)?;
        let d = &self.defs[idx];
        Some(Location {
            uri: self.uri.clone(),
            range: d.selection_range,
        })
    }

    fn hover_info(&self, name: &str, pos: Position) -> Option<String> {
        // If multiple defs exist, choose closest preceding or the one at current position
        let mut best: Option<&SymbolDef> = None;
        let mut best_distance = i64::MAX;

        for d in self.defs.iter().filter(|d| d.name == name) {
            // Check if cursor is within the symbol's range (hovering on definition)
            if pos_in_range(pos, d.selection_range) {
                return Some(d.detail.clone());
            }

            // Otherwise, find closest preceding definition
            if d.defined_at.line < pos.line
                || (d.defined_at.line == pos.line && d.defined_at.character <= pos.character)
            {
                let distance = ((pos.line as i64 - d.defined_at.line as i64) * 1000)
                    + (pos.character as i64 - d.defined_at.character as i64);
                if distance < best_distance {
                    best_distance = distance;
                    best = Some(d);
                }
            }
        }
        best.map(|d| d.detail.clone())
    }

    fn find_references(&self, uri: &str, name: &str) -> Vec<Location> {
        let Ok(url) = Url::parse(uri) else {
            return vec![];
        };
        self.occurrences
            .get(name)
            .into_iter()
            .flatten()
            .cloned()
            .map(|range| Location {
                uri: url.clone(),
                range,
            })
            .collect()
    }

    fn document_symbols(&self) -> Vec<DocumentSymbol> {
        #[allow(deprecated)]
        self.defs
            .iter()
            .map(|d| DocumentSymbol {
                name: d.name.clone(),
                detail: Some(d.detail.clone()),
                kind: d.kind,
                tags: None,
                deprecated: None,
                range: d.full_range,
                selection_range: d.selection_range,
                children: None,
            })
            .collect()
    }
}

fn token_range(text: &str, t: &Token) -> Range {
    let start = Position {
        line: (t.line.saturating_sub(1)) as u32,
        character: (t.col.saturating_sub(1)) as u32,
    };
    // LSP positions are UTF-16 code units
    let len = t.value.encode_utf16().count() as u32;
    let end = Position {
        line: start.line,
        character: start.character + len,
    };
    // cap to line length (defensive)
    let _ = text; // currently unused but kept for future UTF-16 adjustments
    Range { start, end }
}

fn range_start(r: Range) -> Position {
    r.start
}

fn pos_in_range(pos: Position, range: Range) -> bool {
    (pos.line > range.start.line
        || (pos.line == range.start.line && pos.character >= range.start.character))
        && (pos.line < range.end.line
            || (pos.line == range.end.line && pos.character <= range.end.character))
}

fn parse_optional_type(tokens: &[Token], mut i: usize) -> (Option<String>, usize) {
    // expects colon IDENT
    if tokens.get(i).map(|t| t.kind.clone()) == Some(Kind::Colon) {
        i += 1;
        if let Some(t) = tokens.get(i) {
            if t.kind == Kind::Ident {
                return (Some(t.value.clone()), i + 1);
            }
        }
    }
    (None, i)
}

fn infer_initializer_type(tokens: &[Token], mut i: usize) -> Option<String> {
    // scan forward to '=' then look at next literal-ish token
    while i < tokens.len() && tokens[i].kind != Kind::Eq && tokens[i].kind != Kind::Newline {
        i += 1;
    }
    if tokens.get(i).map(|t| t.kind.clone()) != Some(Kind::Eq) {
        return None;
    }
    i += 1;
    let t = tokens.get(i)?;
    match t.kind {
        Kind::Int => Some("int".to_string()),
        Kind::Float => Some("float".to_string()),
        Kind::Str | Kind::Char => Some("string".to_string()),
        Kind::Regex => Some("regex".to_string()),
        Kind::LBracket => Some("array".to_string()),
        Kind::LBrace => Some("object".to_string()),
        Kind::Ident if t.value == "true" || t.value == "false" => Some("bool".to_string()),
        Kind::Ident if t.value == "null" => Some("null".to_string()),
        _ => None,
    }
}

fn parse_function_signature(tokens: &[Token], name_idx: usize) -> (String, usize) {
    let name = tokens
        .get(name_idx)
        .map(|t| t.value.clone())
        .unwrap_or_default();
    // find '(' after name
    let mut i = name_idx + 1;
    while i < tokens.len() && tokens[i].kind != Kind::LParen {
        i += 1;
    }
    let mut params: Vec<String> = Vec::new();
    if i < tokens.len() && tokens[i].kind == Kind::LParen {
        i += 1;
        while i < tokens.len() && tokens[i].kind != Kind::RParen {
            if tokens[i].kind == Kind::Ident {
                let pname = tokens[i].value.clone();
                let (pty, next) = parse_optional_type(tokens, i + 1);
                if let Some(t) = pty {
                    params.push(format!("{pname}: {t}"));
                } else {
                    params.push(pname);
                }
                i = next;
                // skip comma
                if tokens.get(i).map(|t| t.kind.clone()) == Some(Kind::Comma) {
                    i += 1;
                }
                continue;
            }
            i += 1;
        }
        if i < tokens.len() && tokens[i].kind == Kind::RParen {
            i += 1;
        }
    }
    // optional HawkTuah (async marker)
    let is_async = tokens.get(i).map(|t| t.kind.clone()) == Some(Kind::HawkTuah);
    if is_async {
        i += 1;
    }
    let (ret, i2) = parse_optional_type(tokens, i);
    let ret_s = ret.unwrap_or_else(|| "any".to_string());

    // Format signature with return type prominently displayed
    let async_marker = if is_async { " HawkTuah" } else { "" };
    let params_str = if params.is_empty() {
        String::new()
    } else {
        params.join(", ")
    };

    let sig = format!(
        "```rizz\nBruh {}({}){}\n```\n**Returns:** `{}`",
        name, params_str, async_marker, ret_s
    );
    (sig, i2)
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
