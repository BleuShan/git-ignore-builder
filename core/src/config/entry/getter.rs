use super::ConfigEntry;
use crate::prelude::*;
use git2::Config as GitConfig;

pub trait ConfigEntryGetter: ConfigEntry {
    fn get(&self, source: GitConfig) -> Result<Self::Value>;
}

impl<Entry, Value> ConfigEntryGetter for Entry
where
    Entry: ConfigEntry<Value = Value>,

    Value: FromStr + Sized,
{
    fn get(&self, source: GitConfig) -> Result<Self::Value> {
        let s = source.get_str(Self::NAME.as_str())?;
        s.parse::<Self::Value>()
            .map_err(|_| anyhow!("Failed so parse {}", *Self::NAME))
    }
}
