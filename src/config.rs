pub fn repo_url() -> String {
    String::from(env!("CARGO_PKG_REPOSITORY"))
}

pub fn help_url() -> String {
    format!("{}#help", repo_url())
}

