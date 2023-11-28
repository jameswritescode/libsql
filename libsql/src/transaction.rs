use std::ops::Deref;

use crate::Result;

use super::Connection;

#[derive(Debug)]
pub enum TransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
    ReadOnly,
}

pub struct Transaction {
    pub(crate) inner: Box<dyn Tx + Send + Sync>,
    pub(crate) conn: Connection,
    /// An optional action executed whenever a transaction needs to be dropped.
    pub(crate) close: Option<Box<dyn FnOnce() -> ()>>,
}

impl Transaction {
    pub async fn commit(mut self) -> Result<()> {
        self.inner.commit().await
    }

    pub async fn rollback(mut self) -> Result<()> {
        self.inner.rollback().await
    }
}

impl Deref for Transaction {
    type Target = Connection;

    #[inline]
    fn deref(&self) -> &Connection {
        &self.conn
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Some(close) = self.close.take() {
            close();
        }
    }
}

#[async_trait::async_trait]
pub(crate) trait Tx {
    async fn commit(&mut self) -> Result<()>;
    async fn rollback(&mut self) -> Result<()>;
}
