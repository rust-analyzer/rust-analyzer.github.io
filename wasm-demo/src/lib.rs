#![cfg(target_arch = "wasm32")]
#![allow(non_snake_case)]

use ra_ide_api::{Analysis, FileId, FilePosition};
use ra_syntax::SyntaxKind;
use wasm_bindgen::prelude::*;

mod conv;
use conv::*;
mod return_types;
use return_types::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).expect("could not install logging hook");
    log::info!("worker initialized")
}

#[wasm_bindgen]
pub struct WorldState {
    analysis: Analysis,
    file_id: FileId,
}

#[wasm_bindgen]
impl WorldState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (analysis, file_id) = Analysis::from_single_file("".to_owned());
        Self { analysis, file_id }
    }

    pub fn update(&mut self, code: String) -> JsValue {
        log::warn!("update");
        let (analysis, file_id) = Analysis::from_single_file(code);
        self.analysis = analysis;
        self.file_id = file_id;

        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let highlights: Vec<_> = self
            .analysis
            .highlight(file_id)
            .unwrap()
            .into_iter()
            .map(|hl| Highlight { tag: Some(hl.tag), range: hl.range.conv_with(&line_index) })
            .collect();

        let diagnostics: Vec<_> = self
            .analysis
            .diagnostics(self.file_id)
            .unwrap()
            .into_iter()
            .map(|d| {
                let Range { startLineNumber, startColumn, endLineNumber, endColumn } =
                    d.range.conv_with(&line_index);
                Diagnostic {
                    message: d.message,
                    severity: d.severity.conv(),
                    startLineNumber,
                    startColumn,
                    endLineNumber,
                    endColumn,
                }
            })
            .collect();

        serde_wasm_bindgen::to_value(&UpdateResult { diagnostics, highlights }).unwrap()
    }

    pub fn completions(&self, line_number: u32, column: u32) -> JsValue {
        log::warn!("completions");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let pos = Position { line_number, column }.conv_with((&line_index, self.file_id));
        let res = match self.analysis.completions(pos).unwrap() {
            Some(items) => items,
            None => return JsValue::NULL,
        };

        let items: Vec<_> = res.into_iter().map(|item| item.conv_with(&line_index)).collect();
        serde_wasm_bindgen::to_value(&items).unwrap()
    }

    pub fn hover(&self, line_number: u32, column: u32) -> JsValue {
        log::warn!("hover");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let pos = Position { line_number, column }.conv_with((&line_index, self.file_id));
        let info = match self.analysis.hover(pos).unwrap() {
            Some(info) => info,
            _ => return JsValue::NULL,
        };

        let value = info.info.to_markup();
        let hover = Hover {
            contents: vec![MarkdownString { value }],
            range: info.range.conv_with(&line_index),
        };

        serde_wasm_bindgen::to_value(&hover).unwrap()
    }

    pub fn code_lenses(&self) -> JsValue {
        log::warn!("code_lenses");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let results: Vec<_> = self
            .analysis
            .file_structure(self.file_id)
            .unwrap()
            .into_iter()
            .filter(|it| match it.kind {
                SyntaxKind::TRAIT_DEF | SyntaxKind::STRUCT_DEF | SyntaxKind::ENUM_DEF => true,
                _ => false,
            })
            .filter_map(|it| {
                let position =
                    FilePosition { file_id: self.file_id, offset: it.node_range.start() };
                let nav_info = self.analysis.goto_implementation(position).unwrap()?;

                let title = if nav_info.info.len() == 1 {
                    "1 implementation".into()
                } else {
                    format!("{} implementations", nav_info.info.len())
                };

                let positions = nav_info
                    .info
                    .iter()
                    .map(|target| target.focus_range().unwrap_or(target.full_range()))
                    .map(|range| range.conv_with(&line_index))
                    .collect();

                Some(CodeLensSymbol {
                    range: it.node_range.conv_with(&line_index),
                    command: Some(Command {
                        id: "editor.action.showReferences".into(),
                        title,
                        positions,
                    }),
                })
            })
            .collect();

        serde_wasm_bindgen::to_value(&results).unwrap()
    }

    pub fn references(&self, line_number: u32, column: u32) -> JsValue {
        log::warn!("references");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let pos = Position { line_number, column }.conv_with((&line_index, self.file_id));
        let info = match self.analysis.find_all_refs(pos).unwrap() {
            Some(info) => info,
            _ => return JsValue::NULL,
        };

        let res: Vec<_> = info
            .into_iter()
            .map(|r| Highlight { tag: None, range: r.range.conv_with(&line_index) })
            .collect();
        serde_wasm_bindgen::to_value(&res).unwrap()
    }

    pub fn prepare_rename(&self, line_number: u32, column: u32) -> JsValue {
        log::warn!("prepare_rename");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let pos = Position { line_number, column }.conv_with((&line_index, self.file_id));
        let refs = match self.analysis.find_all_refs(pos).unwrap() {
            None => return JsValue::NULL,
            Some(refs) => refs,
        };

        let declaration = refs.declaration();
        let range = declaration.range().conv_with(&line_index);
        let text = declaration.name().to_string();

        serde_wasm_bindgen::to_value(&RenameLocation { range, text }).unwrap()
    }

    pub fn rename(&self, line_number: u32, column: u32, new_name: &str) -> JsValue {
        log::warn!("rename");
        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let pos = Position { line_number, column }.conv_with((&line_index, self.file_id));
        let change = match self.analysis.rename(pos, new_name) {
            Ok(Some(change)) => change,
            _ => return JsValue::NULL,
        };

        let result: Vec<_> = change
            .info
            .source_file_edits
            .iter()
            .flat_map(|sfe| sfe.edit.as_atoms())
            .map(|atom| atom.conv_with(&line_index))
            .collect();

        serde_wasm_bindgen::to_value(&result).unwrap()
    }
}
