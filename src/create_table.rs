use std::fmt::Write;

use crate::create_column::{CreateColumn, CreateColumnImpl};
use crate::error::Error;
use crate::{DBImpl, Value};

/// The builder for a `CREATE TABLE` statement
pub struct CreateTableBuilder<'until_build, 'post_build> {
    pub(crate) name: &'until_build str,
    pub(crate) columns: Vec<CreateColumnImpl<'until_build, 'post_build>>,
    pub(crate) if_not_exists: bool,
    pub(crate) lookup: Vec<Value<'post_build>>,
    pub(crate) pre_statements: Vec<(String, Vec<Value<'post_build>>)>,
    pub(crate) statements: Vec<(String, Vec<Value<'post_build>>)>,
    pub(crate) dialect: DBImpl,
}

impl<'until_build, 'post_build> CreateTableBuilder<'until_build, 'post_build> {
    /// Add a column to the table
    pub fn add_column(mut self, column: CreateColumnImpl<'until_build, 'post_build>) -> Self {
        self.columns.push(column);
        self
    }

    /// Sets the IF NOT EXISTS trait on the table
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// This method is used to convert the current state for the given dialect in a list of tuples.
    ///
    /// Each tuple consists of a query string and the corresponding bind parameters.
    pub fn build(mut self) -> Result<Vec<(String, Vec<Value<'post_build>>)>, Error> {
        match self.dialect {
            #[cfg(feature = "sqlite")]
            DBImpl::SQLite => {
                let mut s = format!(
                    "CREATE TABLE{} \"{}\" (",
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name
                );

                let columns_len = self.columns.len() - 1;
                for (idx, mut x) in self.columns.into_iter().enumerate() {
                    #[cfg(any(feature = "mysql", feature = "postgres"))]
                    if let CreateColumnImpl::SQLite(ref mut cci) = x {
                        cci.statements = Some(&mut self.statements)
                    }
                    #[cfg(not(any(feature = "mysql", feature = "postgres")))]
                    {
                        let CreateColumnImpl::SQLite(ref mut cci) = x;
                        cci.statements = Some(&mut self.statements);
                    }

                    x.build(&mut s)?;

                    if idx != columns_len {
                        write!(s, ", ").unwrap();
                    }
                }

                write!(s, ") STRICT; ").unwrap();

                let mut statements = vec![(s, self.lookup)];
                statements.extend(self.statements);

                Ok(statements)
            }
            #[cfg(feature = "mysql")]
            DBImpl::MySQL => {
                let mut s = format!(
                    "CREATE TABLE{} `{}` (",
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name
                );

                let columns_len = self.columns.len() - 1;
                for (idx, mut x) in self.columns.into_iter().enumerate() {
                    #[cfg(any(feature = "postgres", feature = "sqlite"))]
                    if let CreateColumnImpl::MySQL(ref mut cci) = x {
                        cci.statements = Some(&mut self.statements);
                    }
                    #[cfg(not(any(feature = "postgres", feature = "sqlite")))]
                    {
                        let CreateColumnImpl::MySQL(ref mut cci) = x;
                        cci.statements = Some(&mut self.statements);
                    }

                    x.build(&mut s)?;

                    if idx != columns_len {
                        write!(s, ", ").unwrap();
                    }
                }

                write!(s, "); ").unwrap();

                let mut statements = vec![(s, self.lookup)];
                statements.extend(self.statements);

                Ok(statements)
            }
            #[cfg(feature = "postgres")]
            DBImpl::Postgres => {
                let mut s = format!(
                    "CREATE TABLE{} \"{}\" (",
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name
                );

                let columns_len = self.columns.len() - 1;
                for (idx, mut x) in self.columns.into_iter().enumerate() {
                    #[cfg(any(feature = "sqlite", feature = "mysql"))]
                    if let CreateColumnImpl::Postgres(ref mut cci) = x {
                        cci.pre_statements = Some(&mut self.pre_statements);
                        cci.statements = Some(&mut self.statements);
                    }
                    #[cfg(not(any(feature = "sqlite", feature = "mysql")))]
                    {
                        let CreateColumnImpl::Postgres(ref mut cci) = x;
                        cci.pre_statements = Some(&mut self.pre_statements);
                        cci.statements = Some(&mut self.statements);
                    }

                    x.build(&mut s)?;

                    if idx != columns_len {
                        write!(s, ", ").unwrap();
                    }
                }

                write!(s, "); ").unwrap();

                let mut statements = self.pre_statements;
                statements.push((s, self.lookup));
                statements.extend(self.statements);

                Ok(statements)
            }
        }
    }
}
