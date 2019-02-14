#[cfg(test)]

use dotenv;
use r2d2::CustomizeConnection;

pub fn init_env() {
    dotenv::from_filename(".env.test").ok();
}

#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::connection::Connection,
    E: std::error::Error + Sync + Send,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        match conn.begin_test_transaction() {
            Ok(_) => Ok(()),
            Err(_) => Ok(()), // TODO: Fix this with real error
        }
    }
}
