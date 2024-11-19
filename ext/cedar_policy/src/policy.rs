use std::{collections::HashMap};

use magnus::{function, method, scan_args::{scan_args, get_kwargs}, Error, Module, Object, RModule, Ruby, Value};

use cedar_policy::{Policy, PolicyId};

use crate::error::PARSE_ERROR;


#[magnus::wrap(class = "CedarPolicy::Policy")]
pub struct RPolicy(Policy);

impl RPolicy {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::< _, (), (), (), _, ()>(args)?;
        let (policy_str,): (String,) = args.required;
        let kw_args = get_kwargs::<_, (), (Option<Value>,), ()>(args.keywords, &[], &["id"])?;
        let (policy_id,): (Option<Value>,) = kw_args.optional;

        let policy_id = policy_id.map(|policy_id| PolicyId::new(policy_id.to_string()));
        Policy::parse(policy_id, policy_str)
            .map(|policy| Self(policy) )
            .map_err(|err|
                Error::new(
                    ruby.get_inner(&PARSE_ERROR),
                    format!("Unable to parse policy: {}", err.to_string())
                )
            )
    }

    fn id(&self) -> String {
        self.0.id().to_string()
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn annotations(&self) -> HashMap<String, String> {
        // let annotationHash = HashMap<String, Option<String>>
        self.0.annotations().into_iter()
            .map(|(key, value)| (String::from(key), String::from(value)))
            .collect()
    }
}

impl From<&Policy> for RPolicy {
    fn from(policy: &Policy) -> Self {
        Self(policy.clone())
    }
}

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("Policy", ruby.class_object())?;
    class.define_singleton_method("new", function!(RPolicy::new, -1))?;
    class.define_method("id", method!(RPolicy::id, 0))?;
    class.define_method("to_s", method!(RPolicy::to_string, 0))?;
    class.define_method("annotations", method!(RPolicy::annotations, 0))?;

    Ok(())
}
