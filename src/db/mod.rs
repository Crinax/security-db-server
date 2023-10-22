pub mod orm;

pub trait DbUrlProvider {
    fn db_url(&self) -> &str;
}

pub struct DbBuilder {
    db_url: Option<String>
}

impl DbBuilder {
    fn new() -> Self {
        Self {
            db_url: None
        }
    }

    fn bind(&mut self, url_provider: impl DbUrlProvider) -> &mut Self {
        // Solve it as &str if it's possible
        self.db_url = Some(url_provider.db_url().to_string());

        self
    }
}
