use std::path::PathBuf;

pub fn get_tempaltes_files(file_name: &str) -> PathBuf{

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("src")
    .join("html_templates")
    .join(file_name)

}