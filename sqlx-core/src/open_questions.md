# Open Questions

## Goals
1. Figure out how to specify and account for DB backend.
1. Figure out how to store, and append to the sql statement.
1. Figure out how to store, and append to the arguments list.
1. Figure out how to build the whole thing into a query.

## Methodology

In order to understand how we can build a Query from sql strings and bind arguments
for a given db backend, it will help to understand how query(), bind(), and
execute() build a Query from sql strings, bind arguments, and a db pool.

1. How does query work?
  1. How does it build Query?

1. How does Query get the db type?
  1. How/Where does DB_POOL hold DB type 
  1. How does execute pull out DB type and provide it to Query?

1. How do arguments work?
  1. How are Arguments defined by bind?   
  1. How does bind pass Query arguments?

## Answers
1. How does query build Query?
  1. query makes a Query with `query<DB>(sql: &str)`
     It sets database as PhantomData
     It sets just wraps sql as `Either::Left(sql)`

1. How does execute() pull out the DB type and provide it to Query?
  1. It takes an Executor and calls `executor.execute(self)` on the query
  1. Executor.execute() calls `self.execute_many(query)...` 

  1. It doesn't really, it just forwards the query to the database.
 
1. How/Where does DB_POOL, connection hold DB type?

1. How are Arguments defined by bind?
1. How does bind pass Query arguments?
  1. Bind takes `value: T` and calls `arguments.add(value)`
  1. Query creates arguments as `Some(Default::default())`

## Conclusions
1. (tentative) We can store the sql statments as &str or String.
1. 


# Code

```
17     pub async fn find(id: i32) -> Result<Self, sqlx::Error> {
18         let q = sqlx::query_as::<_, Game>(
19             r#"
20         SELECT *
21         FROM games
22         WHERE id = $1
23                 "#
24         );
25
26         let b = q.bind(id);
27
28         let f = b.fetch_one(&*DB_POOL);
29         let a = f.await;
30
31         a
32     }
```

q is QueryAs
b is QueryAs
f is an Opaque Type: `Result<O, Error>`
a is a Game
