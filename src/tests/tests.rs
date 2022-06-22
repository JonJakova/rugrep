#[cfg(test)]
mod tests {

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive"],
            crate::search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            crate::search_case_sensitive(query, contents)
        );
    }
}
