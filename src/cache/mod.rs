use redis::{Client, Connection};

pub enum CacheError<T> {
    ConnectionOpen,
    ConnectionGet,
    Execution(T),
    AddPair,
    ExpireSet,
    GetPair,
}

pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new(url: &str) -> Result<Self, CacheError<()>> {
        Ok(
            Self {
                client: Client::open(url)
                    .map_err(|_| CacheError::ConnectionOpen)?,
            }
        )
    }

    pub fn apply<T, E>(&self, clojure: impl Fn(&mut Connection) -> Result<T, E>) -> Result<T, CacheError<E>> {
       match self.client.get_connection() {
           Ok(mut connection) => match clojure(&mut connection) {
               Ok(result) => Ok(result),
               Err(err) => Err(CacheError::Execution(err)),
           },
           Err(_) => Err(CacheError::ConnectionGet)
       }
    }

    pub fn add_pair(&self, key: &str, value: &str, ttl: usize) -> Result<bool, CacheError<CacheError<()>>> {
        self.apply(|conn| {
            redis::cmd("SET").arg(key).arg(value).query(conn)
                .map_err(|_| CacheError::AddPair)?;
            redis::cmd("EXPIREAT").arg(ttl).query(conn)
                .map_err(|_| CacheError::ExpireSet)?;

            Ok(true)
        })
    }

    pub fn get_pair(&self, key: &str) -> Result<Option<String>, CacheError<CacheError<()>>> {
        self.apply(|conn| {
            let value: Option<String> = redis::cmd("GET").arg(key).query(conn)
                .map_err(|_| CacheError::GetPair)?;

            Ok(value)
        })
    }
}
