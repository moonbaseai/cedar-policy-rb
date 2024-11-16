use std::{ops::Deref, str::FromStr};

use cedar_policy::{CedarSchemaError, Schema};
use magnus::{function, scan_args::scan_args, value::ReprValue, Error, Module, Object, RModule, Ruby, TryConvert, Value};

use crate::error::PARSE_ERROR;

#[magnus::wrap(class = "CedarPolicy::Schema")]
pub struct RSchema(Schema);

impl RSchema {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), _, (), (), (), ()>(args)?;
        let (schema,): (Option<String>,) = args.optional;

        match schema {
            Some(schema) => Self::from_str(&schema)
                .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string())),
            None => Err(Error::new(ruby.get_inner(&PARSE_ERROR), "you must supply schema contents")),
        }
    }
}

impl From<RSchema> for Schema {
    fn from(schema: RSchema) -> Self {
        schema.0.clone()
    }
}

impl From<&RSchema> for Schema {
    fn from(schema: &RSchema) -> Self {
        schema.0.clone()
    }
}

impl FromStr for RSchema {
    type Err = CedarSchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Schema::from_str(s)?))
    }
}

// impl TryConvert for RSchema {
//     fn try_convert(value: Value) -> Result<Self, Error> {
//         let handle = Ruby::get_with(value);
//         match <RSchema>::try_convert(value) {
//             Ok(value) => Ok(value),
//             Err(_) => Err(Error::new(handle.exception_arg_error(), "Unabled to convert Value to RSchema")),
//         }
//     }
// }

// impl TryConvert for RSchema {
//     // type Error = CedarSchemaError;

//     fn try_convert(value: Value) -> Result<Self, Error> {
//         let handle = Ruby::get_with(value);
//         let schema: Result<RSchema, Error> = if value.class() == RSchema {
//             value.try_into()
//         };
//         match value.respond_to("schema", false) {
//             Ok(true) => {
//                 let schema: Value = value.funcall_public("schema", ())?;
//                 let schema = match schema {}
//             }
//             Err(e) => Err(Error::new(handle.exception_arg_error(), e.to_string())),
//             _ => Err(Error::new(
//                 handle.exception_arg_error(),
//                 format!("no implicit conversion of {} into Entities", unsafe {
//                     value.classname()
//                 }),
//             ))?,
//         }
//     }
// }

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Schema", ruby.class_object())?;
    class.define_singleton_method("new", function!(RSchema::new, -1))?;

    Ok(())
}
