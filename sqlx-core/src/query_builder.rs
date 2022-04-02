use std::fmt::Display;

use crate::arguments::IntoArguments;
use crate::database::{Database, HasArguments};
use crate::encode::Encode;
use crate::query::Query;
use either::Either;
use std::marker::PhantomData;

pub struct QueryBuilder<'a, DB>
where
    DB: Database,
    // A: IntoArguments<'a, DB>,
{
    query: String,
    buf: <DB as HasArguments<'a>>::ArgumentBuffer,
    // q: Query<'a, DB, A>,
}

impl<'a, DB: Database> QueryBuilder<'a, DB>
where
    DB: Database,
    // A: IntoArguments<'a, DB> + std::default::Default,
{
    pub fn new(init: impl Into<String>) -> Self
    where
        <DB as HasArguments<'a>>::ArgumentBuffer: Default,
    {
        QueryBuilder {
            query: init.into(),
            buf: Default::default(),
            // q: Query {
            //     statement: either::Left(&init.into()),
            //     arguments: Some(Default::default()),
            //     database: PhantomData,
            //     persistent: true,
            // },
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.query.push_str(&format!(" {}", sql));

        self
    }

    pub fn push_bind(&mut self, value: impl Encode<'a, DB>) -> &mut Self {
        value.encode(&mut self.buf);

        self
    }

    // pub fn build(&mut self) -> Query<'static, DB, A> {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::postgres::Postgres;

    struct MyQuerySegment {
        sql: String,
    }

    impl Display for MyQuerySegment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.sql)
        }
    }

    #[test]
    fn test_new() {
        let first_line = "SELECT * FROM users";
        let qb: QueryBuilder<'_, Postgres> = QueryBuilder::new(first_line);
        assert_eq!(first_line, qb.query);
    }

    #[test]
    fn test_push() {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT * FROM users");
        let second_line = "WHERE last_name LIKE '[A-N]%;";

        assert_eq!(
            "SELECT * FROM users WHERE last_name LIKE '[A-N]%;".to_string(),
            qb.push(second_line).query
        );
    }

    #[test]
    fn test_push_bind() {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT * FROM users");
        let second_line = "WHERE last_name.id = ?";
        let value: i32 = 42;

        assert_eq!(*qb.push_bind(value).buf, vec![0, 0, 0, 42u8]);
    }
}
