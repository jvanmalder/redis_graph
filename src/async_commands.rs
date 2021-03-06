use crate::types::*;
use redis::aio::ConnectionLike;
use redis::{cmd, RedisFuture, ToRedisArgs};

/// Provides a high level asynchronous API to work with Redis graph data types.
/// The graph command becomes directly available on ConnectionLike types from
/// the redis crate when you import the GraphCommands trait.
/// ```rust,no_run
/// # async fn run() -> redis::RedisResult<()> {
/// use redis::AsyncCommands;
/// use redis_graph::{AsyncGraphCommands, GraphResultSet};
///
/// let client = redis::Client::open("redis://127.0.0.1/")?;
/// let mut con = client.get_async_connection().await?;
///
/// let res:GraphResultSet = con.graph_query(
///     "my_graph",
///     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
/// ).await?;
/// # Ok(()) }
/// ```
///
pub trait AsyncGraphCommands: ConnectionLike + Send + Sized {
    fn graph_query<'a, K: ToRedisArgs + Send + Sync + 'a, Q: ToRedisArgs + Send + Sync + 'a>(
        &'a mut self,
        key: K,
        query: Q,
    ) -> RedisFuture<GraphResultSet> {
        Box::pin(async move {
            cmd("GRAPH.QUERY")
                .arg(key)
                .arg(query)
                .query_async(self)
                .await
        })
    }
}

impl<T> AsyncGraphCommands for T where T: Send + ConnectionLike {}
