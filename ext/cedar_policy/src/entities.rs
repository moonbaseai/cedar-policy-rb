use cedar_policy::ffi::JsonValueWithNoDuplicateKeys;
use cedar_policy::{Entities, Schema};
use magnus::{
    value::{Lazy, ReprValue},
    Error, Module, RClass, Ruby, TryConvert, Value,
};
use serde_magnus::deserialize;

use crate::{CEDAR_POLICY, schema::RSchema};

static ENTITIES: Lazy<RClass> = Lazy::new(|ruby| {
    ruby.get_inner(&CEDAR_POLICY)
        .define_class("Entities", ruby.class_object())
        .unwrap()
});

pub struct EntitiesWrapper(Entities);

impl From<EntitiesWrapper> for Entities {
    fn from(value: EntitiesWrapper) -> Self {
        value.0
    }
}

impl TryConvert for EntitiesWrapper {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let handle = Ruby::get_with(value);
        let schema = match value.respond_to("schema", false) {
            Ok(true) => {
                let schema: Value = value.funcall_public("schema", ())?;
                if schema.is_nil() {
                    None
                } else {
                let schema = <&RSchema>::try_convert(schema)?;
                let schema: Schema = schema.into();
                Some(schema)
                }
            },
            _ => None
        };
        match value.respond_to("to_ary", false) {
            Ok(true) => {
                // let schema = value.funcall_public("schema", ())?;
                // let schema = <&RSchema>::try_convert(schema)?;
                // let schema: Schema = schema.into();
                let value: Value = value.funcall_public("to_ary", ())?;
                let value: JsonValueWithNoDuplicateKeys = deserialize(value)?;
                // Ok(Self(
                //     Entities::from_json_value(value.into(), Some(&schema))
                //         .map_err(|e| Error::new(handle.exception_arg_error(), e.to_string()))?,
                // ))
                let entities = match schema {
                    Some(schema) => Entities::from_json_value(value.into(), Some(&schema)),
                    None => Entities::from_json_value(value.into(), None)
                };
                Ok(Self(
                    entities.map_err(|e| Error::new(handle.exception_arg_error(), e.to_string()))?,
                ))
            }
            Err(e) => Err(Error::new(handle.exception_arg_error(), e.to_string())),
            _ => Err(Error::new(
                handle.exception_arg_error(),
                format!("no implicit conversion of {} into Entities", unsafe {
                    value.classname()
                }),
            ))?,
        }
    }
}

pub fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&ENTITIES, ruby);
    Ok(())
}
