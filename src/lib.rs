#[allow(warnings)]
mod bindings;

use bindings::exports::component::lexopt_wasm::lexopt as lexopt_wasm;
use bindings::exports::component::lexopt_wasm::lexopt::{
    Guest, GuestParser, GuestRawArgs, GuestValuesIter,
};
use std::{cell::RefCell, ffi::OsString, os::wasi::ffi::OsStringExt};

struct Component;
impl Guest for Component {
    type Parser = ComponentParser;
    type RawArgs = ComponentRawArgs;
    type ValuesIter = ComponentValuesIter;
    fn arg_unexpected(self_: lexopt_wasm::Arg) -> lexopt_wasm::Error {
        Into::<lexopt::Arg>::into(self_).unexpected().into()
    }
}

struct ComponentParser(RefCell<lexopt::Parser>);
impl GuestParser for ComponentParser {
    fn bin_name(&self) -> Option<String> {
        self.0.borrow().bin_name().map(Into::into)
    }
    fn from_args(args: Vec<Vec<u8>>) -> lexopt_wasm::Parser {
        lexopt_wasm::Parser::new(Into::<ComponentParser>::into(lexopt::Parser::from_args(
            args.into_iter()
                .map(|x| OsString::from_vec(x))
                .collect::<Vec<_>>(),
        )))
    }
    fn from_env() -> lexopt_wasm::Parser {
        lexopt_wasm::Parser::new(Into::<ComponentParser>::into(lexopt::Parser::from_env()))
    }
    fn from_iter(iter: Vec<Vec<u8>>) -> lexopt_wasm::Parser {
        lexopt_wasm::Parser::new(Into::<ComponentParser>::into(lexopt::Parser::from_iter(
            iter.into_iter().map(|x| OsString::from_vec(x)),
        )))
    }
    fn next(&self) -> Result<Option<lexopt_wasm::Arg>, lexopt_wasm::Error> {
        self.0
            .borrow_mut()
            .next()
            .map(|x| x.map(Into::into))
            .map_err(Into::into)
    }
    fn optional_value(&self) -> Option<Vec<u8>> {
        self.0.borrow_mut().optional_value().map(|x| x.into_vec())
    }
    fn raw_args(&self) -> Result<lexopt_wasm::RawArgs, lexopt_wasm::Error> {
        self.0
            .borrow_mut()
            .raw_args()
            .map(|x| lexopt_wasm::RawArgs::new(Into::<ComponentRawArgs>::into(x)))
            .map_err(Into::into)
    }
    fn try_raw_args(&self) -> Option<lexopt_wasm::RawArgs> {
        self.0
            .borrow_mut()
            .try_raw_args()
            .map(|x| lexopt_wasm::RawArgs::new(Into::<ComponentRawArgs>::into(x)))
    }
    fn value(&self) -> Result<Vec<u8>, lexopt_wasm::Error> {
        self.0
            .borrow_mut()
            .value()
            .map(|x| x.into_vec())
            .map_err(Into::into)
    }
    fn values(&self) -> Result<lexopt_wasm::ValuesIter, lexopt_wasm::Error> {
        self.0
            .borrow_mut()
            .values()
            .map(|x| lexopt_wasm::ValuesIter::new(Into::<ComponentValuesIter>::into(x)))
            .map_err(Into::into)
    }
}
impl From<lexopt::Parser> for ComponentParser {
    fn from(value: lexopt::Parser) -> Self {
        Self(value.into())
    }
}
impl From<ComponentParser> for lexopt::Parser {
    fn from(value: ComponentParser) -> Self {
        value.0.into_inner()
    }
}

struct ComponentRawArgs(RefCell<lexopt::RawArgs<'static>>);
impl GuestRawArgs for ComponentRawArgs {
    fn as_slice(&self) -> Vec<Vec<u8>> {
        self.0
            .borrow()
            .as_slice()
            .into_iter()
            .map(|x| x.clone().into_vec())
            .collect()
    }
    fn peek(&self) -> Option<Vec<u8>> {
        self.0.borrow().peek().map(|x| x.to_owned().into_vec())
    }
}
impl From<lexopt::RawArgs<'_>> for ComponentRawArgs {
    fn from(value: lexopt::RawArgs) -> Self {
        // FIXME: This is bad
        let value: lexopt::RawArgs<'static> = unsafe { std::mem::transmute(value) };
        Self(value.into())
    }
}
impl From<ComponentRawArgs> for lexopt::RawArgs<'_> {
    fn from(value: ComponentRawArgs) -> Self {
        value.0.into_inner()
    }
}

struct ComponentValuesIter(RefCell<lexopt::ValuesIter<'static>>);
impl GuestValuesIter for ComponentValuesIter {}
impl From<lexopt::ValuesIter<'_>> for ComponentValuesIter {
    fn from(value: lexopt::ValuesIter) -> Self {
        // FIXME: This is bad
        let value: lexopt::ValuesIter<'static> = unsafe { std::mem::transmute(value) };
        Self(value.into())
    }
}
impl From<ComponentValuesIter> for lexopt::ValuesIter<'_> {
    fn from(value: ComponentValuesIter) -> Self {
        value.0.into_inner()
    }
}

impl From<lexopt::Arg<'_>> for lexopt_wasm::Arg {
    fn from(value: lexopt::Arg) -> Self {
        match value {
            lexopt::Arg::Long(x) => lexopt_wasm::Arg::Long(x.into()),
            lexopt::Arg::Short(x) => lexopt_wasm::Arg::Short(x.into()),
            lexopt::Arg::Value(x) => lexopt_wasm::Arg::Value(x.into_vec()),
        }
    }
}
impl From<lexopt_wasm::Arg> for lexopt::Arg<'_> {
    fn from(value: lexopt_wasm::Arg) -> Self {
        match value {
            // FIXME: This is a memory leak
            lexopt_wasm::Arg::Long(x) => lexopt::Arg::Long(Box::leak(x.into_boxed_str())),
            lexopt_wasm::Arg::Short(x) => lexopt::Arg::Short(x.into()),
            lexopt_wasm::Arg::Value(x) => lexopt::Arg::Value(OsString::from_vec(x)),
        }
    }
}

impl From<lexopt::Error> for lexopt_wasm::Error {
    fn from(value: lexopt::Error) -> Self {
        match value {
            lexopt::Error::Custom(x) => lexopt_wasm::Error::Custom(x.to_string()),
            lexopt::Error::MissingValue { option } => {
                lexopt_wasm::Error::MissingValue(lexopt_wasm::ErrorMissingValue { option: option })
            }
            lexopt::Error::NonUnicodeValue(x) => lexopt_wasm::Error::NonUnicodeValue(x.into_vec()),
            lexopt::Error::ParsingFailed { value, error } => {
                lexopt_wasm::Error::ParsingFailed(lexopt_wasm::ErrorParsingFailed {
                    value: value,
                    error: error.to_string(),
                })
            }
            lexopt::Error::UnexpectedArgument(x) => {
                lexopt_wasm::Error::UnexpectedArgument(x.into_vec())
            }
            lexopt::Error::UnexpectedOption(x) => lexopt_wasm::Error::UnexpectedOption(x),
            lexopt::Error::UnexpectedValue { option, value } => {
                lexopt_wasm::Error::UnexpectedValue(lexopt_wasm::ErrorUnexpectedValue {
                    option: option,
                    value: value.into_vec(),
                })
            }
        }
    }
}
impl From<lexopt_wasm::Error> for lexopt::Error {
    fn from(value: lexopt_wasm::Error) -> Self {
        match value {
            lexopt_wasm::Error::Custom(x) => lexopt::Error::Custom(x.into()),
            lexopt_wasm::Error::MissingValue(lexopt_wasm::ErrorMissingValue { option }) => {
                lexopt::Error::MissingValue { option: option }
            }
            lexopt_wasm::Error::NonUnicodeValue(x) => {
                lexopt::Error::NonUnicodeValue(OsString::from_vec(x))
            }
            lexopt_wasm::Error::ParsingFailed(lexopt_wasm::ErrorParsingFailed { value, error }) => {
                lexopt::Error::ParsingFailed {
                    value: value,
                    error: error.into(),
                }
            }
            lexopt_wasm::Error::UnexpectedArgument(x) => {
                lexopt::Error::UnexpectedArgument(OsString::from_vec(x))
            }
            lexopt_wasm::Error::UnexpectedOption(x) => lexopt::Error::UnexpectedOption(x),
            lexopt_wasm::Error::UnexpectedValue(lexopt_wasm::ErrorUnexpectedValue {
                option,
                value,
            }) => lexopt::Error::UnexpectedValue {
                option: option,
                value: OsString::from_vec(value),
            },
        }
    }
}

bindings::export!(Component with_types_in bindings);
