use chrono::prelude::*;

#[allow(dead_code)]
pub enum TimestampType {
    Relative,
    ShortTime,
    LongTime,
    ShortDate,
    LongDate,
    ShortDateTime,
    LongDateTime,
}

impl TimestampType {
    pub fn as_char(&self) -> char {
        match self {
            TimestampType::Relative => 'R',
            TimestampType::ShortTime => 't',
            TimestampType::LongTime => 'T',
            TimestampType::ShortDate => 'd',
            TimestampType::LongDate => 'D',
            TimestampType::ShortDateTime => 'f',
            TimestampType::LongDateTime => 'F',
        }
    }
}

/// Converts to Discord timestamp
pub fn timestamp(time: DateTime<Utc>, mode: TimestampType) -> String {
    format!("<t:{}:{}>", time.timestamp(), mode.as_char())
}

/// Pluralizes a unit
pub fn plural(unit: &str, value: i64) -> String {
    format!("{} {}{}", value, unit, if value != 1 { "s" } else { "" })
}
