use sea_orm_migration::{prelude::*, sea_orm::StatementBuilder};

pub trait SchemaTable {
    /// create a new table
    fn create() -> TableCreateStatement;

    /// drop a table
    fn drop() -> TableDropStatement;
}

#[derive(Default)]
pub struct Statements<S>(Vec<S>);

impl<S> Statements<S> {
    pub async fn exec(self, manager: &SchemaManager<'_>) -> Result<(), DbErr>
    where
        S: StatementBuilder,
    {
        for s in self.0 {
            manager.exec_stmt(s).await?;
        }

        Ok(())
    }
}

impl Statements<TableCreateStatement> {
    #[inline]
    pub fn create_table() -> Self {
        Self::default()
    }

    #[inline]
    pub fn push<T: SchemaTable>(mut self) -> Self {
        self.0.push(T::create());
        self
    }
}

impl Statements<TableDropStatement> {
    #[inline]
    pub fn drop_table() -> Self {
        Self::default()
    }

    #[inline]
    pub fn push<T: SchemaTable>(mut self) -> Self {
        self.0.push(T::drop());
        self
    }
}
