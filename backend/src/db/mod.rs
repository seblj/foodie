use futures_core::Future;
use futures_util::TryStreamExt;
use sqlx::{pool::MaybePoolConnection, Error, Executor, PgPool, Postgres, Transaction};

pub mod models;
mod queries;

pub use queries::*;
use sqlx_core::pool::PoolConnection;
use uuid::Uuid;

#[derive(Clone)]
pub struct FoodieDatabase {
    pub pool: PgPool,
}

impl FoodieDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get(self, user_id: Option<Uuid>) -> FoodiePool {
        FoodiePool {
            pool: self.pool,
            user_id,
        }
    }
}
// TODO: Need to find a way to make it impossible to use FoodiePool.pool inside `fetch_one` and
// stuff like that
#[derive(Clone, Debug)]
pub struct FoodiePool {
    pool: PgPool,
    pub user_id: Option<Uuid>,
}

impl FoodiePool {
    pub fn new(pool: PgPool, user_id: Option<Uuid>) -> Self {
        Self { pool, user_id }
    }

    pub async fn begin(&self) -> Result<Transaction<'static, Postgres>, Error> {
        let conn = self.pool.acquire().await?;
        let conn = get_rls_connection(conn, self.user_id).await?;
        Transaction::begin(MaybePoolConnection::PoolConnection(conn)).await
    }

    pub fn close(&self) -> impl Future<Output = ()> + '_ {
        self.pool.close()
    }
}

async fn get_rls_connection(
    mut conn: PoolConnection<Postgres>,
    user_id: Option<Uuid>,
) -> Result<PoolConnection<Postgres>, Error> {
    println!("trying to set id: {:?}", user_id);
    if let Some(user_id) = user_id {
        println!("setting id: {}", user_id);
        sqlx::query(&format!("SET foodie.user_id = '{}'", user_id))
            .execute(&mut *conn)
            .await?;
    }

    Ok(conn)
}

impl<'c> Executor<'c> for &'c FoodiePool {
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_core::stream::BoxStream<
        'e,
        Result<
            itertools::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        let pool = self.pool.clone();

        Box::pin(sqlx_core::ext::async_stream::TryAsyncStream::new(
            move |mut sender| async move {
                macro_rules! r#yield {
                    ($v:expr) => {{
                        let _ = futures_util::sink::SinkExt::send(&mut sender, Ok($v)).await;
                    }};
                }
                let conn = pool.acquire().await?;
                let mut conn = get_rls_connection(conn, self.user_id).await?;

                let mut s = conn.fetch_many(query);
                while let Some(v) = s.try_next().await? {
                    r#yield!(v);
                }
                Ok(())
            },
        ))
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> futures_core::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx::Database>::Row>, Error>,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        let pool = self.pool.clone();

        Box::pin(async move {
            let conn = pool.acquire().await?;
            let mut conn = get_rls_connection(conn, self.user_id).await?;
            conn.fetch_optional(query).await
        })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> futures_core::future::BoxFuture<
        'e,
        Result<<Self::Database as sqlx_core::database::HasStatement<'q>>::Statement, Error>,
    >
    where
        'c: 'e,
    {
        let pool = self.pool.clone();

        Box::pin(async move {
            let conn = pool.acquire().await?;
            let mut conn = get_rls_connection(conn, self.user_id).await?;
            conn.prepare_with(sql, parameters).await
        })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures_core::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, Error>>
    where
        'c: 'e,
    {
        let pool = self.pool.clone();

        Box::pin(async move {
            let conn = pool.acquire().await?;
            let mut conn = get_rls_connection(conn, self.user_id).await?;
            conn.describe(sql).await
        })
    }
}
