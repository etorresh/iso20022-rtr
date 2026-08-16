use std::fs;
use typify::{TypeSpace, TypeSpaceSettings};

// reference: https://github.com/oxidecomputer/typify/blob/main/example-build/build.rs
fn main() {
    let schemas_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/");
    let output_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/generated/");
    let entries = std::fs::read_dir(schemas_dir).unwrap();
    for schema_path in entries.map(|entry| entry.unwrap().path()) {
        let content = fs::read_to_string(&schema_path).unwrap();
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_root_schema(schema).unwrap();

        let contents =
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

        let output_file_stem = schema_path.file_stem().unwrap();
        let output_path = output_dir.join(output_file_stem).with_extension("rs");
        fs::write(output_path, contents).unwrap();
    }
}
