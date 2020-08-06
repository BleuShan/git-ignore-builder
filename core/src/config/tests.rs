#![allow(unused_variables)]

use super::{
    entry::{
        ConfigEntry,
        IgnoreDirectory,
        RemoteUrl,
        WithConfigSetterKind,
    },
    Config,
    GitConfig,
};
use crate::{
    constants::GIT_CONFIG_SECTION_NAME,
    prelude::*,
};
use tempfile::NamedTempFile;

#[fixture]
fn config() -> Config {
    let tempfile = NamedTempFile::new().expect("Failed to create temp config");
    let path = tempfile.path();
    Config::new(path)
}

#[template]
#[rstest(
    entry,
    entry_name,
    expected,
    case::remote_url(
        RemoteUrl,
        format!("{}.{}", GIT_CONFIG_SECTION_NAME, RemoteUrl::SCOPED_NAME),
        "test"
    ),
    case::ignore_directory(
        IgnoreDirectory,
        format!("{}.{}", GIT_CONFIG_SECTION_NAME, IgnoreDirectory::SCOPED_NAME),
        "/test"
    )
)]
fn entry_cases<Entry>(entry: Entry, entry_name: String, expected: &'static str)
where
    Entry: ConfigEntry,
{
}

fn set_str<Entry>(config: &Config, entry: Entry, value: &str)
where
    Entry: ConfigEntry,
{
    let mut writer = match config.path.as_ref() {
        Some(path) => GitConfig::open(path.as_ref()),
        None => GitConfig::open_default(),
    }
    .expect("Failed to open");
    writer
        .set_str(Entry::NAME.as_str(), value)
        .expect("Failed to write var to config");
}

fn get_string<Entry>(config: &Config, entry: Entry) -> String
where
    Entry: ConfigEntry,
{
    let reader = match config.path.as_ref() {
        Some(path) => GitConfig::open(path.as_ref()),
        None => GitConfig::open_default(),
    }
    .and_then(|mut git_config| git_config.snapshot())
    .expect("Failed to open Config");

    reader
        .get_string(Entry::NAME.as_str())
        .expect("Failed to read var from config")
        .to_owned()
}

#[apply(entry_cases)]
fn a_config_entry_should_have_defined_valid_name<Entry>(
    entry: Entry,
    entry_name: String,
    expected: &'static str,
) where
    Entry: ConfigEntry,
{
    expect!(Entry::NAME.clone()).to(be_equal_to(entry_name));
}

#[apply(entry_cases)]
fn a_config_should_be_able_to_load_the_configured_values<Entry, Value, Err>(
    entry: Entry,
    entry_name: String,
    expected: &'static str,
    config: Config,
) where
    Entry: ConfigEntry<Value = Value>,
    Err: Debug,
    Value: Debug + PartialEq + FromStr<Err = Err> + Sized,
{
    let expected_value = expected.parse().expect("failed to parse expected value");
    expect!(config.get(entry)).to(be_err());
    set_str(&config, entry, expected);

    expect!(config.get(entry)).to(be_ok().value(expected_value));
}

#[apply(entry_cases)]
fn a_config_should_be_able_to_load_the_configured_values_or_default<Entry, Value, Err>(
    entry: Entry,
    entry_name: String,
    expected: &'static str,
    config: Config,
) where
    Entry: ConfigEntry<Value = Value>,
    Err: Debug,
    Value: Debug + PartialEq + FromStr<Err = Err> + Sized,
{
    let expected_value = expected.parse().expect("failed to parse expected value");
    expect!(config.get_or_default(entry)).to(be_equal_to(Entry::default_value()));
    set_str(&config, entry, expected);

    expect!(config.get(entry)).to(be_ok().value(expected_value));
}

#[apply(entry_cases)]
fn a_config_should_be_able_to_set_the_configured_values<Entry, Value, Err>(
    entry: Entry,
    entry_name: String,
    expected: &'static str,
    config: Config,
) where
    Entry: ConfigEntry<Value = Value>,
    Err: Debug,
    Value: Debug + PartialEq + WithConfigSetterKind + FromStr<Err = Err> + Sized,
{
    let expected_value = Value::from_str(expected).expect("failed to parse expected value");
    expect!(config.set(entry, expected_value)).to(be_ok());
    expect!(get_string(&config, entry)).to(be_equal_to(expected.to_owned()));
}
