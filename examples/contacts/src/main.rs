#![feature(async_await, try_blocks)]

use failure::Fallible;
use fake::{
    faker::{
        internet::en::{Password, SafeEmail, Username},
        name::en::Name,
        phone_number::en::PhoneNumber,
    },
    Dummy, Fake, Faker,
};
use futures::{channel::oneshot::channel, future, stream::TryStreamExt};
use sqlx::{Postgres, Pool};
use std::{
    io,
    time::{Duration, Instant},
};

type PostgresPool = Pool<Postgres>;

#[derive(Debug, Dummy)]
struct Contact {
    #[dummy(faker = "Name()")]
    name: String,

    #[dummy(faker = "Username()")]
    username: String,

    #[dummy(faker = "Password(5..25)")]
    password: String,

    #[dummy(faker = "SafeEmail()")]
    email: String,

    #[dummy(faker = "PhoneNumber()")]
    phone: String,
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::try_init()?;

    let pool = PostgresPool::new("postgres://postgres@127.0.0.1/sqlx__dev", 85);

    ensure_schema(&pool).await?;
    insert(&pool, 50_000).await?;
    select(&pool, 1_000).await?;

    Ok(())
}

async fn ensure_schema(pool: &PostgresPool) -> io::Result<()> {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS contacts (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL
)
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query("TRUNCATE contacts").execute(&pool).await?;

    Ok(())
}

async fn insert(pool: &PostgresPool, count: usize) -> io::Result<()> {
    let start_at = Instant::now();
    let mut handles = vec![];

    for _ in 0..count {
        let pool = pool.clone();
        let contact: Contact = Faker.fake();
        let (tx, rx) = channel::<()>();

        tokio::spawn(async move {
            sqlx::query(
                r#"
    INSERT INTO contacts (name, username, password, email, phone)
    VALUES ($1, $2, $3, $4, $5)
                    "#,
            )
            .bind(contact.name)
            .bind(contact.username)
            .bind(contact.password)
            .bind(contact.email)
            .bind(contact.phone)
            .execute(&pool)
            .await
            .unwrap();

            tx.send(()).unwrap();
        });

        handles.push(rx);
    }

    future::join_all(handles).await;

    let elapsed = start_at.elapsed();

    println!("insert {} rows in {:?}", count, elapsed);

    Ok(())
}

async fn select(pool: &PostgresPool, iterations: usize) -> io::Result<()> {
    let start_at = Instant::now();
    let mut rows: usize = 0;

    for _ in 0..iterations {
        // TODO: Once we have FromRow derives we can replace this with Vec<Contact>
        let contacts: Vec<(String, String, String, String, String)> = sqlx::query(
            r#"
SELECT name, username, password, email, phone 
FROM contacts
                "#,
        )
        .fetch(&pool)
        .try_collect()
        .await?;

        rows = contacts.len();
    }

    let elapsed = start_at.elapsed();
    let per = Duration::from_nanos((elapsed.as_nanos() / (iterations as u128)) as u64);

    println!(
        "select {} rows in ~{:?} [ x{} in {:?} ]",
        rows, per, iterations, elapsed
    );

    Ok(())
}
