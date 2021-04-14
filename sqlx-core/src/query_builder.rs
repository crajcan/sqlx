pub struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    pub fn new(init: impl Into<String>) -> Self {
        QueryBuilder { query: init.into() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!("", QueryBuilder::new("").query);
    }
}
