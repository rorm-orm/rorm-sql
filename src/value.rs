use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};
use uuid::Uuid;

use crate::cows::VecCow;

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
    ChronoNaiveTime,
    /// Naive Date representation
    ChronoNaiveDate,
    /// Naive DateTime representation
    ChronoNaiveDateTime,
    /// Chrono timezone aware date time representation
    ChronoDateTime,
    /// time's date representation
    TimeDate,
    /// time's time representation
    TimeTime,
    /// time's offset datetime representation
    TimeOffsetDateTime,
    /// time's primitive datetime representation
    TimePrimitiveDateTime,
    /// Uuid representation
    Uuid,
    /// Uuid in hyphenated representation
    UuidHyphenated,
    /// Uuid in simple text representation
    UuidSimple,
    /// serde_json's Value representation
    JsonValue,
    /// Mac address representation
    #[cfg(feature = "postgres-only")]
    MacAddress,
    /// IP network presentation
    #[cfg(feature = "postgres-only")]
    IpNetwork,
    /// Bit vec representation
    #[cfg(feature = "postgres-only")]
    BitVec,
}

/**
This enum represents a value
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    /// null representation
    Null(NullType),
    /// Representation of an identifier, e.g. a column.
    /// This variant will not be escaped, so do not
    /// pass unchecked data to it.
    #[deprecated(note = "Is this still used?")]
    Ident(Cow<'a, str>),
    /// Representation of a column name with
    /// an optional table name
    Column {
        /// Name of the table
        table_name: Option<Cow<'a, str>>,
        /// Name of the column
        column_name: Cow<'a, str>,
    },
    /// Representation of choices
    Choice(Cow<'a, str>),
    /// String representation
    String(Cow<'a, str>),
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
    Binary(Cow<'a, [u8]>),
    /// chrono's Naive Time representation
    ChronoNaiveTime(NaiveTime),
    /// chrono's Naive Date representation
    ChronoNaiveDate(NaiveDate),
    /// chrono's Naive DateTime representation
    ChronoNaiveDateTime(NaiveDateTime),
    /// chrono's Timezone aware datetime
    ChronoDateTime(DateTime<Utc>),
    /// time's date representation
    TimeDate(Date),
    /// time's time representation
    TimeTime(Time),
    /// time's offset datetime representation
    TimeOffsetDateTime(OffsetDateTime),
    /// time's primitive datetime representation
    TimePrimitiveDateTime(PrimitiveDateTime),
    /// Uuid representation
    Uuid(Uuid),
    /// Uuid in hyphenated representation
    #[deprecated(note = "Was this ever used?")]
    UuidHyphenated(Uuid),
    /// Uuid in simple text representation
    #[deprecated(note = "Was this ever used?")]
    UuidSimple(Uuid),
    /// serde_json's Value representation
    JsonValue(&'a serde_json::Value),

    /// Mac address representation
    #[cfg(feature = "postgres-only")]
    MacAddress(mac_address::MacAddress),
    /// IP network presentation
    #[cfg(feature = "postgres-only")]
    IpNetwork(ipnetwork::IpNetwork),
    /// Bit vec representation
    #[cfg(feature = "postgres-only")]
    BitVec(Cow<'a, bit_vec::BitVec>),

    /// null representation
    #[cfg(feature = "postgres-only")]
    ArrayNull(NullType),
    /// String representation
    #[cfg(feature = "postgres-only")]
    ArrayString(VecCow<'a, Cow<'a, str>>),
    /// i64 representation
    #[cfg(feature = "postgres-only")]
    ArrayI64(VecCow<'a, i64>),
    /// i32 representation
    #[cfg(feature = "postgres-only")]
    ArrayI32(VecCow<'a, i32>),
    /// i16 representation
    #[cfg(feature = "postgres-only")]
    ArrayI16(VecCow<'a, i16>),
    /// Bool representation
    #[cfg(feature = "postgres-only")]
    ArrayBool(VecCow<'a, bool>),
    /// f64 representation
    #[cfg(feature = "postgres-only")]
    ArrayF64(VecCow<'a, f64>),
    /// f32 representation
    #[cfg(feature = "postgres-only")]
    ArrayF32(VecCow<'a, f32>),
    /// binary representation
    #[cfg(feature = "postgres-only")]
    ArrayBinary(VecCow<'a, Cow<'a, [u8]>>),
    /// chrono's Naive Time representation
    #[cfg(feature = "postgres-only")]
    ArrayChronoNaiveTime(VecCow<'a, NaiveTime>),
    /// chrono's Naive Date representation
    #[cfg(feature = "postgres-only")]
    ArrayChronoNaiveDate(VecCow<'a, NaiveDate>),
    /// chrono's Naive DateTime representation
    #[cfg(feature = "postgres-only")]
    ArrayChronoNaiveDateTime(VecCow<'a, NaiveDateTime>),
    /// chrono's Timezone aware datetime
    #[cfg(feature = "postgres-only")]
    ArrayChronoDateTime(VecCow<'a, DateTime<Utc>>),
    /// time's date representation
    #[cfg(feature = "postgres-only")]
    ArrayTimeDate(VecCow<'a, Date>),
    /// time's time representation
    #[cfg(feature = "postgres-only")]
    ArrayTimeTime(VecCow<'a, Time>),
    /// time's offset datetime representation
    #[cfg(feature = "postgres-only")]
    ArrayTimeOffsetDateTime(VecCow<'a, OffsetDateTime>),
    /// time's primitive datetime representation
    #[cfg(feature = "postgres-only")]
    ArrayTimePrimitiveDateTime(VecCow<'a, PrimitiveDateTime>),
    /// Uuid representation
    #[cfg(feature = "postgres-only")]
    ArrayUuid(VecCow<'a, Uuid>),
    /// serde_json's Value representation
    #[cfg(feature = "postgres-only")]
    ArrayJsonValue(VecCow<'a, &'a serde_json::Value>),

    /// Mac address representation
    #[cfg(feature = "postgres-only")]
    ArrayMacAddress(VecCow<'a, mac_address::MacAddress>),
    /// IP network presentation
    #[cfg(feature = "postgres-only")]
    ArrayIpNetwork(VecCow<'a, ipnetwork::IpNetwork>),
    /// Bit vec representation
    #[cfg(feature = "postgres-only")]
    ArrayBitVec(VecCow<'a, Cow<'a, bit_vec::BitVec>>),
}

/// [`Value`] should be covariant over `'a`
#[expect(unused)]
fn test_variance<'a, 'b>(x: Value<'a>) -> Value<'b>
where
    'a: 'b,
{
    x
}
