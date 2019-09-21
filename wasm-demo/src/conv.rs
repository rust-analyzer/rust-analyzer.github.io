use super::return_types;
use ra_ide_api::{CompletionItemKind, FileId, FilePosition, LineCol, LineIndex};
use ra_syntax::TextRange;

pub trait Conv {
    type Output;
    fn conv(self) -> Self::Output;
}

pub trait ConvWith<CTX> {
    type Output;
    fn conv_with(self, ctx: CTX) -> Self::Output;
}

#[derive(Clone, Copy)]
pub struct Position {
    pub line_number: u32,
    pub column: u32,
}

impl ConvWith<(&LineIndex, FileId)> for Position {
    type Output = FilePosition;

    fn conv_with(self, (line_index, file_id): (&LineIndex, FileId)) -> Self::Output {
        let line_col = LineCol { line: self.line_number - 1, col_utf16: self.column - 1 };
        let offset = line_index.offset(line_col);
        FilePosition { file_id, offset }
    }
}

impl ConvWith<&LineIndex> for TextRange {
    type Output = return_types::Range;

    fn conv_with(self, line_index: &LineIndex) -> Self::Output {
        let start = line_index.line_col(self.start());
        let end = line_index.line_col(self.end());

        return_types::Range {
            startLineNumber: start.line + 1,
            startColumn: start.col_utf16 + 1,
            endLineNumber: end.line + 1,
            endColumn: end.col_utf16 + 1,
        }
    }
}

impl Conv for CompletionItemKind {
    type Output = return_types::CompletionItemKind;

    fn conv(self) -> <Self as Conv>::Output {
        use return_types::CompletionItemKind::*;
        match self {
            CompletionItemKind::Keyword => Keyword,
            CompletionItemKind::Snippet => Snippet,
            CompletionItemKind::Module => Module,
            CompletionItemKind::Function => Function,
            CompletionItemKind::Struct => Struct,
            CompletionItemKind::Enum => Enum,
            CompletionItemKind::EnumVariant => EnumMember,
            CompletionItemKind::BuiltinType => Struct,
            CompletionItemKind::Binding => Variable,
            CompletionItemKind::Field => Field,
            CompletionItemKind::Trait => Interface,
            CompletionItemKind::TypeAlias => Struct,
            CompletionItemKind::Const => Constant,
            CompletionItemKind::Static => Value,
            CompletionItemKind::Method => Method,
            CompletionItemKind::TypeParam => TypeParameter,
            CompletionItemKind::Macro => Method,
        }
    }
}
