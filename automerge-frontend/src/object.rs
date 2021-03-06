use crate::Value;
use automerge_protocol as amp;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Represents the set of conflicting values for a register in an automerge
/// document.
#[derive(Clone, Debug)]
pub struct Values(pub(crate) HashMap<amp::OpID, Rc<RefCell<Object>>>);

impl Values {
    fn to_value(&self) -> Value {
        self.default_value().borrow().value()
    }

    pub(crate) fn default_value(&self) -> Rc<RefCell<Object>> {
        // TODO this function should return an option instead of doing this
        // unwrap
        let default_op_id = self.default_op_id().unwrap();
        self.0.get(&default_op_id).cloned().unwrap()
    }

    pub(crate) fn update_for_opid(&mut self, opid: amp::OpID, value: Rc<RefCell<Object>>) {
        self.0.insert(opid, value);
    }

    pub(crate) fn default_op_id(&self) -> Option<amp::OpID> {
        let mut op_ids: Vec<&amp::OpID> = self.0.keys().collect();
        op_ids.sort();
        op_ids.reverse();
        #[allow(clippy::map_clone)]
        op_ids.first().map(|oid| *oid).cloned()
    }

    pub(crate) fn conflicts(&self) -> HashMap<amp::OpID, Value> {
        self.0
            .iter()
            .map(|(k, v)| (k.clone(), v.as_ref().borrow().value()))
            .collect()
    }
}

/// Internal data type used to represent the values of an automerge document
#[derive(Clone, Debug)]
pub enum Object {
    Sequence(amp::ObjectID, Vec<Option<Values>>, amp::SequenceType),
    Map(amp::ObjectID, HashMap<String, Values>, amp::MapType),
    Primitive(amp::ScalarValue),
}

impl Object {
    pub(crate) fn value(&self) -> Value {
        match self {
            Object::Sequence(_, vals, seq_type) => {
                match seq_type {
                    amp::SequenceType::List => {
                        Value::Sequence(vals.iter()
                            .filter_map(|v| v.clone().map(|v2| v2.to_value()))
                            .collect())
                    }
                    amp::SequenceType::Text => {
                        Value::Text(vals.iter().map(|e| e.as_ref().map(|e| match e.to_value() {
                            Value::Primitive(amp::ScalarValue::Str(s)) => {
                                if s.chars().count() != 1 {
                                    panic!("Text object with a value which is not a single character")
                                }
                                s.chars().next().unwrap()
                            },
                            _ => panic!("Text object with non character element")
                        }).unwrap()).collect())
                    }
                }
            },
            Object::Map(_, vals, map_type) => Value::Map(
                vals.iter()
                    .map(|(k, v)| (k.to_string(), v.to_value()))
                    .collect(),
                *map_type,
            ),
            Object::Primitive(v) => Value::Primitive(v.clone()),
        }
    }

    // TODO RequestKey is probably out of place here, but it was convenient
    pub(crate) fn default_op_id_for_key(&self, key: amp::RequestKey) -> Option<amp::OpID> {
        match (key, self) {
            // TODO this whole function feels off but this clone especially upsets me
            (amp::RequestKey::Num(i), Object::Sequence(_, vals, _)) => vals
                .get(i as usize)
                .and_then(|v| v.clone().and_then(|inner| inner.default_op_id())),
            (amp::RequestKey::Str(ref s), Object::Map(_, vals, _)) => {
                vals.get(s.as_str()).and_then(|v| v.default_op_id())
            }
            _ => None,
        }
    }

    pub(crate) fn id(&self) -> Option<amp::ObjectID> {
        match self {
            Object::Sequence(oid, _, _) => Some(oid.clone()),
            Object::Map(oid, _, _) => Some(oid.clone()),
            Object::Primitive(..) => None,
        }
    }

    pub(crate) fn obj_type(&self) -> Option<amp::ObjType> {
        match self {
            Object::Sequence(_, _, seq_type) => Some(amp::ObjType::Sequence(*seq_type)),
            Object::Map(_, _, map_type) => Some(amp::ObjType::Map(*map_type)),
            Object::Primitive(..) => None,
        }
    }
}
