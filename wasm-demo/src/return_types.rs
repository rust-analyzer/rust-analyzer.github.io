use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub struct Hover {
    pub range: Range,
    pub contents: Vec<MarkdownString>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Range {
    pub startLineNumber: u32,
    pub startColumn: u32,
    pub endLineNumber: u32,
    pub endColumn: u32,
}

#[derive(Serialize)]
pub struct MarkdownString {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct CodeLensSymbol {
    pub range: Range,
    pub command: Option<Command>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub positions: Vec<Range>, // customized
}

#[derive(Serialize)]
pub struct Highlight {
    pub tag: Option<&'static str>,
    pub range: Range,
}

#[derive(Serialize)]
pub struct TextEdit {
    pub range: Range,
    pub text: String,
}

#[derive(Serialize)]
pub struct UpdateResult {
    pub diagnostics: Vec<Diagnostic>,
    pub highlights: Vec<Highlight>,
}

#[derive(Serialize)]
pub struct Diagnostic {
    pub message: String,
    pub startLineNumber: u32,
    pub startColumn: u32,
    pub endLineNumber: u32,
    pub endColumn: u32,
    pub severity: MarkerSeverity,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum MarkerSeverity {
    Hint = 1,
    Info = 2,
    Warning = 4,
    Error = 8,
}

#[derive(Serialize)]
pub struct RenameLocation {
    pub range: Range,
    pub text: String,
}

#[derive(Serialize)]
pub struct CompletionItem {
    pub label: String,
    pub range: Range,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub insertText: String,
    pub insertTextRules: CompletionItemInsertTextRule,
    pub documentation: Option<MarkdownString>,
    pub filterText: String,
    pub additionalTextEdits: Vec<TextEdit>,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum CompletionItemKind {
    Method = 0,
    Function = 1,
    Constructor = 2,
    Field = 3,
    Variable = 4,
    Class = 5,
    Struct = 6,
    Interface = 7,
    Module = 8,
    Property = 9,
    Event = 10,
    Operator = 11,
    Unit = 12,
    Value = 13,
    Constant = 14,
    Enum = 15,
    EnumMember = 16,
    Keyword = 17,
    Text = 18,
    Color = 19,
    File = 20,
    Reference = 21,
    Customcolor = 22,
    Folder = 23,
    TypeParameter = 24,
    Snippet = 25,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum CompletionItemInsertTextRule {
    None = 0,
    /**
     * Adjust whitespace/indentation of multiline insert texts to
     * match the current line indentation.
     */
    KeepWhitespace = 1,
    /**
     * `insertText` is a snippet.
     */
    InsertAsSnippet = 4,
}

#[derive(Serialize)]
pub struct ParameterInformation {
    pub label: String,
}

#[derive(Serialize)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<MarkdownString>,
    pub parameters: Vec<ParameterInformation>,
}

#[derive(Serialize)]
pub struct SignatureHelp {
    pub signatures: [SignatureInformation; 1],
    pub activeSignature: u32,
    pub activeParameter: Option<usize>,
}

#[derive(Serialize)]
pub struct LocationLink {
    pub originSelectionRange: Range,
    pub range: Range,
    pub targetSelectionRange: Range,
}
