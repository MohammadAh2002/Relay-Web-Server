use relay_web_server::html_templates::get_tempaltes_files;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tempaltes_files_returns_expected_path() {
        let result = get_tempaltes_files("hello.html");

        let expected = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("html_templates")
            .join("hello.html");

        assert_eq!(result, expected, "the function did not return the same file");
    }

    #[test]
    fn get_tempaltes_files_returns_other_path_for_different_name() {
        let result = get_tempaltes_files("404.html");

        let expected = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("html_templates")
            .join("hello.html");

        assert_ne!(result, expected, "the function did not return the diffrenet files");
    }

}
