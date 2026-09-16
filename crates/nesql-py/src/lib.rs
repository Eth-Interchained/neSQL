// SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
// SPDX-License-Identifier: BUSL-1.1
// neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)

//! PyO3 bindings: the neSQL command surface for Python.
//!
//! Same practice as the engine's nedb-py crate — the Rust crate is the one
//! implementation; the binding exposes it verbatim. Every call returns the
//! command's JSON body (the same object `nesql --json` prints); errors raise
//! NesqlError carrying the CLI's human text and exit-code name.

use pyo3::exceptions::PyRuntimeError;
use nedb_engine::Db;
use pyo3::prelude::*;

use std::path::PathBuf;
use std::sync::Arc;

fn open_db(path: Option<String>) -> PyResult<(Arc<Db>, PathBuf)> {
    let p = path
        .or_else(|| std::env::var("NEDB_PATH").ok())
        .ok_or_else(|| PyRuntimeError::new_err(
            "a db path is required (pass db_path or set NEDB_PATH)"))?;
    let pathbuf = PathBuf::from(&p);
    let db = Db::open(&pathbuf, None)
        .map(Arc::new)
        .map_err(|e| PyRuntimeError::new_err(format!("open failed: {e}")))?;
    Ok((db, pathbuf))
}

fn report_to_py(report: nesql::out::Report) -> PyResult<String> {
    Ok(report.render("query", nesql::out::Format::Json))
}

/// The neSQL command surface, as a Python object over an open store.
#[pyclass]
struct Nesql {
    db: Arc<Db>,
    #[allow(dead_code)]
    path: PathBuf,
}

#[pymethods]
impl Nesql {
    #[new]
    #[pyo3(signature = (path=None))]
    fn new(path: Option<String>) -> PyResult<Self> {
        let (db, p) = open_db(path)?;
        Ok(Nesql { db, path: p })
    }

    /// Run a neSQL query (SQL or the FROM-form; routed structurally).
    #[pyo3(signature = (q, forced=None))]
    fn query(&self, q: &str, forced: Option<&str>) -> PyResult<String> {
        let forced_dialect = forced
            .map(|f| match f.to_ascii_uppercase().as_str() {
                "NQL" => Ok(nesql::args::Dialect::Nql),
                "SQL" => Ok(nesql::args::Dialect::Sql),
                other => Err(PyRuntimeError::new_err(format!(
                    "forced dialect must be \"nql\" or \"sql\", got {other:?}"))),
            })
            .transpose()?;
        report_to_py(nesql::cmd::query::run_with(&self.db, q, forced_dialect))
    }

    /// `nesql status` — the store's state at a glance.
    fn status(&self) -> PyResult<String> {
        let r = nesql::cmd::status::run(&self.db, &self.path);
        Ok(r.human)
    }

    /// `nesql root verify` — stored record vs fresh recomputation.
    #[pyo3(signature = (at=None))]
    fn root_verify(&self, at: Option<u64>) -> PyResult<String> {
        let r = nesql::cmd::root::verify(&self.db, at);
        Ok(r.human)
    }

    /// `nesql version` — CLI + engine version, grammar digest.
    #[staticmethod]
    fn version() -> String {
        nesql::cmd::version::run().human
    }
}

/// Module-level convenience mirroring the CLI's one-shot shape.
#[pyfunction]
#[pyo3(signature = (q, db_path=None, forced=None))]
fn query(q: &str, db_path: Option<String>, forced: Option<&str>) -> PyResult<String> {
    let (db, _p) = open_db(db_path)?;
    let forced_dialect = forced
        .map(|f| match f.to_ascii_uppercase().as_str() {
            "NQL" => Ok(nesql::args::Dialect::Nql),
            "SQL" => Ok(nesql::args::Dialect::Sql),
            other => Err(PyRuntimeError::new_err(format!(
                "forced dialect must be \"nql\" or \"sql\", got {other:?}"))),
        })
        .transpose()?;
    report_to_py(nesql::cmd::query::run_with(&db, q, forced_dialect))
}


/// `nesql version` — the CLI's version, the engine's, and the grammar digest.
#[pyfunction]
pub fn version() -> String {
    nesql::cmd::version::run().human
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(query, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<Nesql>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
