use std::{str::FromStr, collections::HashSet};

use cedar_policy::{CedarSchemaError, Schema};
use magnus::{function, method, scan_args::scan_args, Error, Module, Object, RArray, RModule, Ruby, Value};

use crate::{error::PARSE_ERROR, entity_uid::to_euid_value};

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

    pub fn principals(&self) -> RArray {
        let principals = self.0.principals().collect::<HashSet<_>>();
        RArray::from_iter(principals.iter().map(|principal| principal.to_string()))
    }

    pub fn resources(&self) -> RArray {
        let resources = self.0.resources().collect::<HashSet<_>>();
        RArray::from_iter(resources.iter().map(|resource| resource.to_string()))
    }

    pub fn action_groups(&self) -> RArray {
        RArray::from_iter(self.0.action_groups().map(|action_group|
            to_euid_value(action_group)
        ))
    }

    pub fn actions(&self) -> RArray {
        RArray::from_iter(self.0.actions().map(|action|
            to_euid_value(action)
        ))
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

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Schema", ruby.class_object())?;
    class.define_singleton_method("new", function!(RSchema::new, -1))?;
    class.define_method("principals", method!(RSchema::principals, 0))?;
    class.define_method("resources", method!(RSchema::resources, 0))?;
    class.define_method("action_groups", method!(RSchema::action_groups, 0))?;
    class.define_method("actions", method!(RSchema::actions, 0))?;

    Ok(())
}
