use std::str::FromStr;

use cedar_policy::{ParseErrors, PolicyId, PolicySet};
use magnus::{function, method, scan_args::{scan_args, get_kwargs}, Error, Module, Object, RArray, RModule, Ruby, Value};

use crate::{error::PARSE_ERROR, policy::RPolicy};

#[magnus::wrap(class = "CedarPolicy::PolicySet")]
pub struct RPolicySet(PolicySet);

impl RPolicySet {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), _, (), (), _, ()>(args)?;
        let (policy,): (Option<String>,) = args.optional;
        let kw_args = get_kwargs::<_, (), (Option<Value>,), ()>(args.keywords, &[], &["id_annotation"])?;
        let (id_annotation,) = kw_args.optional;

        match policy {
            Some(policy) => {
                let policy_set = PolicySet::from_str(&policy)
                    .map_err(|e| Error::new(ruby.get_inner(&PARSE_ERROR), e.to_string()));
                if policy_set.is_err() || id_annotation.is_none() {
                    policy_set.map(|ps| Self(ps) )
                }
                else
                {
                    // Attempt to rename policies by the value of the specified annotation
                    let new_policy_set = Self::rewrite_ids_from_annotation(
                        policy_set.unwrap(), id_annotation.unwrap().to_string()
                    );
                    match new_policy_set {
                        Ok(policy_set) => Ok(Self(policy_set)),
                        Err(err) => Err(Error::new(ruby.exception_arg_error(), err.0))
                    }
                }
            },
            None => Ok(Self(PolicySet::new())),
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn policies(&self) -> RArray {
        RArray::from_iter(self.0.policies().map(|policy| RPolicy::from(policy)))
    }

    // Returns a new policy set where all of the policies that have the specified annotation_name
    // have the value of that annotation used as their id. Any policies that don't have the annotation
    // will  retain their original id.
    fn rewrite_ids_from_annotation(policy_set: PolicySet, annotation_name: String) -> Result<PolicySet, PolicyError> {
        let mut new_policy_set = PolicySet::new();
        let policies = policy_set.policies().map(|policy|
            match policy.annotation(&annotation_name) {
                Some(annotation) => policy.new_id(PolicyId::new(annotation)),
                None => policy.clone()
            }
        );
        for policy in policies {
            let policy_id = policy.id().to_string();
            if new_policy_set.add(policy).is_err() {
                let message = format!("failed to add policy with id {} (duplicate?)", policy_id);
                return Err(PolicyError(message));
            }
        }
        Ok(new_policy_set)
    }
}

impl From<&RPolicySet> for PolicySet {
    fn from(policy: &RPolicySet) -> Self {
        policy.0.clone()
    }
}

impl FromStr for RPolicySet {
    type Err = ParseErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PolicySet::from_str(s)?))
    }
}

// This error exists to communicate errors when renaming policies by annotation values.
#[derive(Debug, Clone)]
struct PolicyError(String);

pub fn init(ruby: &Ruby, module: &RModule) -> Result<(), Error> {
    let class = module.define_class("PolicySet", ruby.class_object())?;
    class.define_singleton_method("new", function!(RPolicySet::new, -1))?;
    class.define_method("empty?", method!(RPolicySet::is_empty, 0))?;
    class.define_method("policies", method!(RPolicySet::policies, 0))?;

    Ok(())
}
