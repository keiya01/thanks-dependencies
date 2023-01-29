pub(super) struct DependencyContent {
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) repository: Option<String>,
}

impl DependencyContent {
    pub(super) fn into_string(self) -> String {
        format!(
            "{}{}{}",
            self.name,
            self.repository
                .map_or_else(|| "".to_owned(), |r| format!("({r})")),
            self.description
                .map_or_else(|| "".to_owned(), |d| format!(" ... {d}"))
        )
    }
}
