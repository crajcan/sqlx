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
    variable_count: u16,
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
            variable_count: 1,
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.query.push_str(&format!(" {}", sql));

        self
    }

    pub fn push_bind(&mut self, value: impl Encode<'a, DB>) -> &mut Self {
        self.query.push_str(&format!("${}", self.variable_count));
        value.encode(&mut self.buf);

        self.variable_count += 1;

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
        let mut qb: QueryBuilder<'_, Postgres> =
            QueryBuilder::new("SELECT * FROM users WHERE id = ");

        qb.push_bind(42i32)
            .push("OR user.membership_level = ")
            .push_bind(3i32);

        assert_eq!(
            qb.query,
            "SELECT * FROM users WHERE id = $1 OR user.membership_level = $2"
        );
        assert_eq!(*qb.buf, vec![0, 0, 0, 42u8, 0, 0, 0, 3u8]);
    }

    // #[test]
    // fn test_build() {
    //     let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT * FROM users")
    //         .push("WHERE id = $1")
    //         .push_bind(42);

    //     assert_eq!(
    //         "SELECT * FROM users WHERE last_name LIKE '[A-N]%;".to_string(),
    //         qb.push(second_line).query
    //     );
    // }
}
