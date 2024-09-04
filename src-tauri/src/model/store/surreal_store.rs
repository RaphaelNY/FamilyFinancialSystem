//! Small store layer to talk to the SurrealDB.
//!
//! This module is to narrow and normalize the surrealdb API surface
//! to the rest of the application code (.e.g, Backend Model Controllers)

use crate::model::store::surreal_modql::build_select_query;
use crate::model::store::{Creatable, Patchable};
use crate::prelude::*;
use crate::utils::{map, XTake};
use crate::{Error, Result};
use modql::filter::FilterGroups;
use modql::ListOptions;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};

// surrealstore 封装了surrealdb的database和session来规范crud操作
pub (in crate::model) struct SurrealStore {
	ds: Datastore,
	ses: Session,
}
impl SurrealStore {
	pub (in crate::model) fn new(ds: Datastore, ses: Session) -> Self {
		Self { ds, ses }
	}

	pub(in crate::model) fn create<T: Creatable>(&self, t: T) -> Result<T> {
		let thing = t.to_thing()?;
		let thing = self.ds.create(&self.ses, thing)?;
		T::from_thing(thing)
	}
}
