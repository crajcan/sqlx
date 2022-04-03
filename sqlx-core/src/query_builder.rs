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
{
    query: String,
    buf: Option<<DB as HasArguments<'a>>::ArgumentBuffer>,
    variable_count: u16,
}

impl<'a, DB: Database> QueryBuilder<'a, DB>
where
    DB: Database,
{
    pub fn new(init: impl Into<String>) -> Self
    where
        <DB as HasArguments<'a>>::ArgumentBuffer: Default,
    {
        QueryBuilder {
            query: init.into(),
            buf: Some(Default::default()),
            variable_count: 0,
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.query.push_str(&format!(" {}", sql));

        self
    }

    pub fn push_bind(&mut self, value: impl Encode<'a, DB>) -> &mut Self {
        match self.buf {
            Some(ref mut buf) => {
                value.encode(buf);
                self.variable_count += 1;
                self.query.push_str(&format!("${}", self.variable_count));
            }
            None => panic!("Arguments taken already"),
        }

        self
    }

    pub fn build(&mut self) -> Query<'_, DB, <DB as HasArguments<'a>>::ArgumentBuffer> {
        let arugments = if let Some(buffer) = self.buf.take() {
            Some(buffer)
        } else {
            None
        };

        Query {
            statement: Either::Left(&self.query),
            arguments: arugments,
            database: PhantomData,
            persistent: true,
        }
    }
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
        let mut qb: QueryBuilder<'_, Postgres> =
            QueryBuilder::new("SELECT * FROM users WHERE id = ");

        qb.push_bind(42i32)
            .push("OR user.membership_level = ")
            .push_bind(3i32);

        assert_eq!(
            qb.query,
            "SELECT * FROM users WHERE id = $1 OR user.membership_level = $2"
        );
        assert_eq!(*qb.buf.unwrap(), vec![0, 0, 0, 42u8, 0, 0, 0, 3u8]);
    }

    #[test]
    fn test_build() {
        let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT * FROM users");

        qb.push("WHERE id = ").push_bind(42i32);
        let query = qb.build();

        assert_eq!(
            query.statement.unwrap_left(),
            "SELECT * FROM users WHERE id = $1"
        );
        assert_eq!(query.arguments.unwrap().as_slice(), vec![0, 0, 0, 42u8]);
        assert_eq!(query.persistent, true);
    }
}
