use relay_web_server::webroot::webroothandler::get_files;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file() {
        let result = get_files("main.html");

        assert!(result.is_ok());

        let result = get_files("404.html");

        assert!(result.is_ok());
    }

    #[test]
    fn get_non_exting_file() {
        let result = get_files("wrong.html");

        assert!(result.is_err());

        let result = get_files(" ");

        assert!(result.is_err());

        let result = get_files("   ");

        assert!(result.is_err());
    }

    #[test]
    fn get_empty_file_path() {
        let result = get_files("");

        assert!(result.is_err());

    }

    #[test]
    fn reject_parent_directory_traversal() {
        let result = get_files("../main.html");

        assert!(result.is_err());
    }

    #[test]
    fn reject_nested_parent_directory_traversal() {
        let result = get_files("images/../../main.html");

        assert!(result.is_err());
    }

    #[test]
    fn reject_absolute_path() {
        let result = get_files("/etc/passwd");

        assert!(result.is_err());
    }

    #[test]
    fn reject_directory_path() {
        let result = get_files("images");

        assert!(result.is_err());
    }

    #[test]
    fn get_file_with_leading_slash() {
        let result = get_files("/main.html");

        assert!(result.is_ok());
    }

}
