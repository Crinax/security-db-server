use redis::{Client, Connection, ToRedisArgs};

pub enum CacheError<T> {
    ConnectionOpen,
    ConnectionGet,
    Execution(T),
    AddPair,
    ExpireSet
}

pub struct Cache {
    client: Client,
}

impl Cache {
    fn new(url: &str) -> Result<Self, CacheError<()>> {
        Ok(
            Self {
                client: Client::open(url)
                    .map_err(|_| CacheError::ConnectionOpen)?,
            }
        )
    }

    fn apply<T, E>(&self, clojure: impl Fn(&mut Connection) -> Result<T, E>) -> Result<T, CacheError<E>> {
       match self.client.get_connection() {
           Ok(mut connection) => match clojure(&mut connection) {
               Ok(result) => Ok(result),
               Err(err) => Err(CacheError::Execution(err)),
           },
           Err(_) => Err(CacheError::ConnectionGet)
       }
    }

    fn add_pair(&self, key: &str, value: &str, ttl: usize) -> Result<bool, CacheError<CacheError<()>>> {
        self.apply(|conn| {
            redis::cmd("SET").arg(key).arg(value).query(conn)
                .map_err(|_| CacheError::AddPair)?;
            redis::cmd("EXPIREAT").arg(ttl).query(conn)
                .map_err(|_| CacheError::ExpireSet)?;

            Ok(true)
        })
    }

    fn get_pair(&self, key: &str) -> Result<Option<String>, CacheError<CacheError<()>>> {
        self.apply(|conn| {
            redis::cmd("GET").arg(key).query(conn)
                .map_err(|_| CacheError::ExpireSet)?;
        })
    }
}
