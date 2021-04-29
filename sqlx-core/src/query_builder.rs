use std::marker::PhantomData;

use std::fmt::Display;
use crate::encode::Encode;
use crate::describe::Describe;
use crate::query::query;
use crate::arguments::{Arguments, IntoArguments};
use crate::database::Database;

#[cfg(feature = "mysql")]
use crate::mysql::MySql;

pub struct QueryBuilder<'q, DB, A> {
    pub(crate) statement: &'q str,
    pub(crate) arguments: Option<A>,
    pub(crate) database:  PhantomData<DB>,
}

impl<'q, DB, A> QueryBuilder<'q, DB, A>
where
    DB: Database,
    A: IntoArguments<'q, DB> + Default,
{
    pub fn new(init: impl Into<String>) -> Self {
        QueryBuilder {
            statement: &init.into(),
            arguments: Some(Default::default()),
            database: PhantomData
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.statement = &format!("{}{}", self.statement, sql);

        self
    }
/*
    //figure out how arguments works
    pub fn push_bind(&mut self, value: impl Encode<DB>) -> Self {
        value.encode(self.params);

        self
    }

    pub fn build(&mut self) -> sqlx::Query<DB, 'static> {
        query("Placeholder string")
    }
*/
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "mysql")]
    use crate::mysql::MySql;

    #[test]
    fn test_new<MySql, A>() {
        assert_eq!("".to_string(), QueryBuilder::new("").statement);
        assert_eq!(Some(Default::default()), QueryBuilder::<MySql, A>::new("").arguments);
    }
/*
    #[test]
    fn test_push() {
        assert_eq!(
            "SELECT * FROM foo".to_string(),
            QueryBuilder::new("").push("SELECT * FROM foo").statement
        );
    }

    #[test]
    fn test_push_bind() {
        let builder = QueryBuilder::new("");

        const EXPECTED: &[u8] = b"D\0\0\0\x0EPsqlx_p_5\0";

        let m = Describe::Portal(5);

        builder.push_bind(m);

        assert_eq!(EXPECTED, builder.params);
    }
*/
}
