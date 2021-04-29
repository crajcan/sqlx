# Open Questions


## Goals
1. Figure out how to specify and account for DB backend.
1. Figure out how to store, and append to the sql statement.
1. Figure out how to store, and append to the arguments list.
1. Figure out how to build the whole thing into a query.

## Methodology

In order to understand how we can build a Query from sql strings and bind arguments
for a given db backend, it will help to understand how query_as(), bind(), and
fetch() build a Query from sql strings, bind arguments, and a db pool.

1. How does query_as work?
  1. How does it build Query?
  1. How does it provide sql to query?

1. How does Query get the db type?
  1. How/Where does DB_POOL hold DB type 
  1. How does fetch pull out DB type and provide it to Query?

1. How do arguments work?
  1. How are Arguments defined by bind?   
  1. How does bind pass Query arguments?

## Answers
1. How does query_as build Query?
  1. It makes a QueryAs with `innner: query(sql)`
  1. query makes a Query with `query<DB>(sql: &str)`, and it sets database as
     PhantomData
1. How does it provide sql to Query?
  1. query_as() passes sql to query().
  1. query() wraps `sql: &str` in `Either::Left(sql)` and assignes it to statement.

1. How/Where does DB_POOL, connection hold DB type?
1. How does fetch pull out the DB type and provide it to Query?

1. How are Arguments defined by bind?
1. How does bind pass Query arguments?

## Conclusions
1. (tentative) We can store the sql statments as &str or String.

