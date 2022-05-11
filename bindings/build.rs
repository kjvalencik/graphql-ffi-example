use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let schema = graphql_ffi_example_server::schema();
    let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let schema_path = {
        let mut path = PathBuf::from(out_dir);
        path.push("schema.graphql");
        path
    };

    let mut file = File::create(schema_path).unwrap();

    file.write_all(schema.as_schema_language().as_bytes())
        .unwrap();
}
