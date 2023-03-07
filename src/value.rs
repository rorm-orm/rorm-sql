use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// This enum represents a [Null](Value::Null)'s type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NullType {
    /// String representation
    String,
    /// Choice representation
    Choice,
    /// i64 representation
    I64,
    /// i32 representation
    I32,
    /// i16 representation
    I16,
    /// Bool representation
    Bool,
    /// f64 representation
    F64,
    /// f32 representation
    F32,
    /// binary representation
    Binary,
    /// Naive Time representation
    NaiveTime,
    /// Naive Date representation
    NaiveDate,
    /// Naive DateTime representation
    NaiveDateTime,
}

/**
This enum represents a value
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value<'a> {
    /// null representation
    Null(NullType),
    /// Representation of an identifier, e.g. a column.
    /// This variant will not be escaped, so do not
    /// pass unchecked data to it.
    Ident(&'a str),
    /// Representation of a column name with
    /// an optional table name
    Column {
        /// Name of the table
        table_name: Option<&'a str>,
        /// Name of the column
        column_name: &'a str,
    },
    /// Representation of choices
    Choice(&'a str),
    /// String representation
    String(&'a str),
    /// i64 representation
    I64(i64),
    /// i32 representation
    I32(i32),
    /// i16 representation
    I16(i16),
    /// Bool representation
    Bool(bool),
    /// f64 representation
    F64(f64),
    /// f32 representation
    F32(f32),
    /// binary representation
    Binary(&'a [u8]),
    /// Naive Time representation
    NaiveTime(NaiveTime),
    /// Naive Date representation
    NaiveDate(NaiveDate),
    /// Naive DateTime representation
    NaiveDateTime(NaiveDateTime),
}
