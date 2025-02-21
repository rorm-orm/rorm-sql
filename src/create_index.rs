use crate::error::Error;
use crate::DBImpl;

/// The builder for a `CREATE INDEX` statement
pub struct CreateIndexBuilder<'until_build> {
    pub(crate) name: &'until_build str,
    pub(crate) table_name: &'until_build str,
    pub(crate) unique: bool,
    pub(crate) if_not_exists: bool,
    pub(crate) columns: Vec<&'until_build str>,
    pub(crate) condition: Option<String>,
    pub(crate) dialect: DBImpl,
}

impl<'until_build> CreateIndexBuilder<'until_build> {
    /// Creates a unique index.
    ///
    /// Null values are considered different from all other null values.
    fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Creates the index only if it doesn't exist yet.
    fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// Adds a column to the index.
    ///
    /// **Parameter**:
    /// - `column`: String representing the column to index.
    fn add_column(mut self, column: &'until_build str) -> Self {
        self.columns.push(column);
        self
    }

    /// Sets the condition to apply. This will build a partial index.
    ///
    /// **Parameter**:
    /// - `condition`: String representing condition to apply the index to
    fn set_condition(mut self, condition: String) -> Self {
        self.condition = Some(condition);
        self
    }

    /// This method is used to build the create index operation.
    fn build(self) -> Result<String, Error> {
        match self.dialect {
            #[cfg(feature = "sqlite")]
            DBImpl::SQLite => {
                if self.columns.is_empty() {
                    return Err(Error::SQLBuildError(format!(
                        "Couldn't create index on {}: Missing column(s) to create the index on",
                        self.table_name
                    )));
                }

                Ok(format!(
                    "CREATE {} INDEX{} {} ON {} ({}) {};",
                    if self.unique { "UNIQUE" } else { "" },
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name,
                    self.table_name,
                    self.columns.join(", "),
                    self.condition.as_ref().map_or("", |x| x.as_str()),
                ))
            }
            #[cfg(feature = "mysql")]
            DBImpl::MySQL => {
                if self.columns.is_empty() {
                    return Err(Error::SQLBuildError(format!(
                        "Couldn't create index on {}: Missing column(s) to create the index on",
                        self.table_name
                    )));
                }

                Ok(format!(
                    "CREATE {} INDEX{} {} ON {} ({});",
                    if self.unique { "UNIQUE" } else { "" },
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name,
                    self.table_name,
                    self.columns.join(", "),
                ))
            }
            #[cfg(feature = "postgres")]
            DBImpl::Postgres => {
                if self.columns.is_empty() {
                    return Err(Error::SQLBuildError(format!(
                        "Couldn't create index on {}: Missing column(s) to create the index on",
                        self.table_name
                    )));
                }

                Ok(format!(
                    "CREATE{} INDEX{} {} ON {} ({}){};",
                    if self.unique { " UNIQUE" } else { "" },
                    if self.if_not_exists {
                        " IF NOT EXISTS"
                    } else {
                        ""
                    },
                    self.name,
                    self.table_name,
                    self.columns.join(", "),
                    match self.condition {
                        None => String::from(""),
                        Some(cond) => format!(" WHERE {}", cond.as_str()),
                    }
                ))
            }
        }
    }
}
