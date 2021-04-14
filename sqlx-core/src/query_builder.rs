use std::fmt::Display;

pub struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    pub fn new(init: impl Into<String>) -> Self {
        QueryBuilder { query: init.into() }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.query = format!("{}{}", self.query, sql);

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!("".to_string(), QueryBuilder::new("").query);
    }

    #[test]
    fn test_push() {
        assert_eq!(
            "SELECT * FROM foo".to_string(),
            QueryBuilder::new("").push("SELECT * FROM foo").query
        );
    }
}
