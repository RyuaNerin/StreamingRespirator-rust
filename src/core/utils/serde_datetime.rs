// https://github.com/egg-mode-rs/egg-mode/blob/6b81073eba9c3b123ca0e80bdb5ef61d1758f131/src/common/mod.rs#L487

use serde::{Serializer, Deserialize, Deserializer};
use serde::de::Error;
use chrono::{DateTime, TimeZone, Utc};

const DATE_FORMAT: &'static str = "%a %b %d %T %z %Y";

pub fn deserialize<'de, D>(ser: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(ser)?;
    let date = Utc
        .datetime_from_str(&s, DATE_FORMAT)
        .map_err(|e| D::Error::custom(e))?;
    Ok(date)
}

pub fn serialize<S>(src: &DateTime<Utc>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.collect_str(&src.format(DATE_FORMAT))
}
