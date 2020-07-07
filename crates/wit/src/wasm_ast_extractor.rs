use walrus::{IdsToIndices, ModuleConfig};

struct WasmAst {
    exports: Vec<wit_support::AstFunctionItem>,
    imports: Vec<wit_support::AstExternModItem>,
    records: Vec<wit_support::AstRecordItem>,
}

pub(crate) fn wasm_ast_extractor(wasm_path: std::path::PathBuf) -> Result<WasmAst, std::io::Error> {
    let module = ModuleConfig::new().parse_file(wasm_path)?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name().starts_with(wit_support::GENERATED_SECTION_NAME))
        .map(|section|)
        .collect::<Vec<_>>();

}
