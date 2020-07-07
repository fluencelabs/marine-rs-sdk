use walrus::ModuleConfig;

#[derive(Default, Clone)]
struct WasmAst {
    exports: Vec<wit_support::AstFunctionItem>,
    imports: Vec<wit_support::AstExternModItem>,
    records: Vec<wit_support::AstRecordItem>,
}

pub(crate) fn wasm_ast_extractor(
    wasm_path: std::path::PathBuf,
) -> Result<Vec<wit_support::FCEAst>, std::io::Error> {
    let module = ModuleConfig::new().parse_file(wasm_path).unwrap();
    let mut decoded_ast = Vec::new();

    for custom_module in module.customs.iter().filter(|(_, section)| {
        section
            .name()
            .starts_with(wit_support::GENERATED_SECTION_NAME)
    }) {
        let default_ids = walrus::IdsToIndices::default();
        let raw_data = custom_module.1.data(&default_ids);
        let decoded_json: wit_support::FCEAst = serde_json::from_slice(&raw_data).unwrap();
        decoded_ast.push(decoded_json);
    }

    Ok(decoded_ast)
}
