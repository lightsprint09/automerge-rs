use crate::time;
use crate::{
    ActorID, AutomergeError, Change, ChangeRequest, ChangeRequestType, Clock, OpID, OpSet,
    Operation, Patch, PendingDiff, Version,
};
use crate::protocol::OpRequest;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone)]
pub struct Backend {
    versions: Vec<Version>,
    op_set: OpSet,
    obj_alias: HashMap<String, OpID>,
}

impl Backend {
    pub fn init() -> Backend {
        let mut versions = Vec::new();
        versions.push(Version {
            version: 0,
            local_only: true,
            op_set: OpSet::init(),
        });
        Backend {
            versions,
            op_set: OpSet::init(),
            obj_alias: HashMap::new(),
        }
    }

    fn process_request(
        &mut self,
        request: &ChangeRequest,
        op_set: &OpSet,
        start_op: u64,
    ) -> Result<Change, AutomergeError> {
        let time = time::unix_timestamp();
        let actor_id = request.actor.clone();
        let obj_alias = &mut self.obj_alias;
        let operations = request.ops.as_ref().map(|ops| ops.iter().enumerate().map(|(n,rop)| {
                let op_num = start_op + (n as u64);
                rop_to_op(rop.clone(), op_num, &actor_id, op_set, obj_alias)
           }).collect()).unwrap_or_default();
        Ok(Change {
            start_op,
            message: request.message.clone(),
            actor_id: request.actor.clone(),
            seq: request.seq,
            deps: request
                .deps
                .clone()
                .ok_or(AutomergeError::InvalidChangeRequest)?,
            time,
            operations,
        })
    }

    fn make_patch(
        &self,
        pending: Vec<PendingDiff>,
        request: Option<&ChangeRequest>,
        incremental: bool,
    ) -> Result<Patch, AutomergeError> {
        let diffs = self.op_set.finalize_diffs(pending)?;

        Ok(Patch {
            version: self.versions.last().map(|v| v.version).unwrap_or(0),
            can_undo: self.op_set.can_undo(),
            can_redo: self.op_set.can_redo(),
            diffs,
            clock: if incremental {
                None
            } else {
                Some(self.op_set.clock.clone())
            },
            actor: request.map(|r| r.actor.clone()),
            seq: request.map(|r| r.seq),
        })
    }

    pub fn undo(
        &mut self,
        request: &ChangeRequest,
        start_op: u64,
    ) -> Result<Change, AutomergeError> {
        let undo_pos = self.op_set.undo_pos;

        if undo_pos < 1 || self.op_set.undo_stack.len() < undo_pos {
            return Err(AutomergeError::InvalidChange(
                "Cannot undo: there is nothing to be undone".to_string(),
            ));
        }

        let undo_ops = self.op_set.undo_stack.remove(undo_pos - 1);

        let redo_ops = Vec::new();
        // FIXME TODO - translate undo ops into redo ops

        let change = Change {
            actor_id: request.actor.clone(),
            seq: request.seq,
            start_op: start_op,
            deps: request
                .deps
                .clone()
                .ok_or(AutomergeError::InvalidChangeRequest)?,
            message: request.message.clone(),
            time: time::unix_timestamp(),
            operations: undo_ops.clone(),
        };

        self.op_set.undo_pos -= 1;
        self.op_set.redo_stack.push(redo_ops);

        Ok(change)
    }

    pub fn redo(
        &mut self,
        request: &ChangeRequest,
        start_op: u64,
    ) -> Result<Change, AutomergeError> {
        let change = Change {
            actor_id: request.actor.clone(),
            seq: request.seq,
            start_op: start_op,
            deps: request
                .deps
                .clone()
                .ok_or(AutomergeError::InvalidChangeRequest)?,
            message: request.message.clone(),
            time: time::unix_timestamp(),
            operations: self
                .op_set
                .redo_stack
                .pop()
                .ok_or(AutomergeError::InvalidChange("no redo ops".to_string()))?,
        };

        self.op_set.undo_pos += 1;

        Ok(change)
    }

    pub fn load_changes(&mut self, changes: Vec<Change>) -> Result<Patch, AutomergeError> {
        self.apply(changes, None, false, false)
    }

    pub fn apply_changes(&mut self, changes: Vec<Change>) -> Result<Patch, AutomergeError> {
        self.versions.iter_mut().for_each(|v| v.local_only = false);
        self.apply(changes, None, false, true)
    }

    pub fn get_version(&self, version: u64) -> Result<&Version, AutomergeError> {
        self.versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| AutomergeError::UnknownVersion(version))
    }

    fn apply(
        &mut self,
        mut changes: Vec<Change>,
        request: Option<&ChangeRequest>,
        undoable: bool,
        incremental: bool,
    ) -> Result<Patch, AutomergeError> {
        let mut diffs = Vec::new();

        for change in changes.drain(0..) {
            self.op_set
                .add_change(change, request.is_some(), undoable, &mut diffs)?;
        }

        //        let diffs2 = self.op_set.finalize_diffs(diffs); // FIXME

        if incremental {
            let version = self.versions.last().map(|v| v.version).unwrap_or(0) + 1;
            let version_obj = Version {
                version,
                local_only: true,
                op_set: self.op_set.clone(),
            };
            self.versions.push(version_obj);
        } else {
            let version_obj = Version {
                version: 0,
                local_only: true,
                op_set: self.op_set.clone(),
            };
            self.versions.clear();
            self.versions.push(version_obj);
        }

        self.make_patch(diffs, request, true)
    }

    pub fn apply_local_change(
        &mut self,
        mut request: ChangeRequest,
    ) -> Result<Patch, AutomergeError> {
        self.op_set.check_for_duplicate(&request)?; // Change has already been applied

        let version = self.get_version(request.version)?.clone();

        let actor = request.actor.clone();
        request
            .deps
            .get_or_insert_with(|| version.op_set.deps.without(&actor));

        let start_op = self.op_set.max_op + 1;
        let change = match request.request_type {
            ChangeRequestType::Change => self.process_request(&request, &version.op_set, start_op)?,
            ChangeRequestType::Undo => self.undo(&request, start_op)?,
            ChangeRequestType::Redo => self.redo(&request, start_op)?,
        };

        let undoable = (request.request_type == ChangeRequestType::Change && request.undoable);

        let patch = self.apply(vec![change.clone()], Some(&request), undoable, true)?;

        self.finalize_version(request.version, change)?;

        Ok(patch)

        //        Ok(self.make_patch(diffs.unwrap(), Some(&tmp_request), true)?)
    }

    fn finalize_version(
        &mut self,
        request_version: u64,
        change: Change,
    ) -> Result<(), AutomergeError> {
        // remove all versions older than this one
        // i wish i had drain filter
        let mut i = 0;
        while i != self.versions.len() {
            if self.versions[i].version < request_version {
                self.versions.remove(i);
            } else {
                i += 1;
            }
        }

        for v in self.versions.iter_mut() {
            if v.local_only {
                v.op_set = self.op_set.clone()
            } else {
                v.op_set = self.op_set.clone();
                v.op_set
                    .add_change(change.clone(), true, false, &mut Vec::new())?; // FIXME - should be passing None for diffs
            }
        }

        Ok(())
    }

    pub fn undo_stack(&self) -> &Vec<Vec<Operation>> {
        &self.op_set.undo_stack
    }

    pub fn redo_stack(&self) -> &Vec<Vec<Operation>> {
        &self.op_set.redo_stack
    }

    pub fn history(&self) -> Vec<&Change> {
        self.op_set
            .states
            .history
            .iter()
            .map(|rc| rc.as_ref())
            .collect()
    }

    pub fn get_patch(&self) -> Patch {
        panic!("not implemented");
        //const diffs = OpSet.constructObject(state.get('opSet'), OpSet.ROOT_ID)
        //return makePatch(state, diffs, null, false)
    }

    /// Get changes which are in `other` but not in this backend
    pub fn get_changes<'a>(&self, other: &'a Backend) -> Result<Vec<&'a Change>, AutomergeError> {
        if self.clock().divergent(&other.clock()) {
            return Err(AutomergeError::DivergedState(
                "Cannot diff two states that have diverged".to_string(),
            ));
        }
        Ok(other.op_set.get_missing_changes(&self.op_set.clock))
    }

    pub fn get_changes_for_actor_id(&self, actor_id: &ActorID) -> Vec<&Change> {
        self.op_set.states.get(actor_id)
    }

    pub fn get_missing_changes(&self, clock: Clock) -> Vec<&Change> {
        self.op_set.get_missing_changes(&clock)
    }

    pub fn get_missing_deps(&self) -> Clock {
        self.op_set.get_missing_deps()
    }

    pub fn merge(&mut self, remote: &Backend) -> Result<Patch, AutomergeError> {
        let missing_changes = remote
            .get_missing_changes(self.op_set.clock.clone())
            .iter()
            .cloned()
            .cloned()
            .collect();
        self.apply_changes(missing_changes)
    }

    pub fn clock(&self) -> &Clock {
        &self.op_set.clock
    }
}

fn rop_to_op(
        rop: OpRequest,
        seq_num: u64,
        actor: &ActorID,
        op_set: &OpSet,
        obj_alias: &mut HashMap<String, OpID>,
    ) -> Operation {
        match rop {
            OpRequest::MakeMap { obj, key, child } => {
                let opid = OpID::parse(&obj).unwrap_or_else(|| obj_alias.get(&obj).unwrap().clone());
                let this = OpID::ID(seq_num, actor.0.clone());
                obj_alias.insert(child, this);
                Operation::MakeMap {
                    object_id: opid,
                    key,
                    pred: Vec::new(),
                }
            }
            OpRequest::Set {
                obj,
                key,
                value,
                insert,
            } => {
                let opid = OpID::parse(&obj).unwrap_or_else(|| obj_alias.get(&obj).unwrap().clone());
                let pred = op_set.get_ops(&opid, &key).unwrap_or_default();
                Operation::Set {
                    object_id: opid,
                    key,
                    value,
                    insert,
                    pred,
                    datatype: None,
                }
            }
        }
    }
