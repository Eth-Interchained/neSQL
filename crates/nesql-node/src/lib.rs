// SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
// SPDX-License-Identifier: BUSL-1.1
// neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)

//! napi-rs bindings: the neSQL command surface for Node.
//!
//! Same practice as the engine's nedb-node crate — the Rust crate is the one
//! implementation; the binding exposes it verbatim. Every call returns the
//! command's JSON body (the same object `nesql --json` prints) and the exit
//! code as a named verdict; errors cross the boundary as Err(String) carrying
//! the CLI's human text, never a panic.

use nedb_engine::Db;
use napi::bindgen_prelude::*;
use napi_derive::napi;

use std::path::PathBuf;
use std::sync::Arc;

fn open_db(path: Option<String>) -> Result<(Arc<Db>, PathBuf)> {
    let p = path
        .or_else(|| std::env::var("NEDB_PATH").ok())
        .ok_or_else(|| Error::new(Status::GenericFailure,
            "a db path is required (pass dbPath or set NEDB_PATH)"))?;
    let pathbuf = PathBuf::from(&p);
    let db = Db::open(&pathbuf, None)
        .map(Arc::new)
        .map_err(|e| Error::new(Status::GenericFailure, format!("open failed: {e}")))?;
    Ok((db, pathbuf))
}

fn report_to_value(report: nesql::out::Report) -> Result<String> {
    // The exact JSON `nesql --json <cmd>` prints: same envelope, same verdict.
    let rendered = report.render("query", nesql::out::Format::Json);
    Ok(rendered)
}

/// Which half of neSQL a statement is written in.
#[napi(string_enum)]
pub enum Dialect { Nql, Sql }

/// `nesql query` — run neSQL (SQL or the FROM-form; routed structurally by the engine).
#[napi]
pub fn query(db_path: Option<String>, q: String, forced: Option<Dialect>) -> Result<String> {
    let (db, _p) = open_db(db_path)?;
    let forced_dialect = forced.map(|d| match d {
        Dialect::Nql => nesql::args::Dialect::Nql,
        Dialect::Sql => nesql::args::Dialect::Sql,
    });
    report_to_value(nesql::cmd::query::run_with(&db, &q, forced_dialect))
}

/// `nesql version` — the CLI's version, the engine's, and the grammar digest.
#[napi]
pub fn version() -> String {
    let r = nesql::cmd::version::run();
    r.human
}

/// `nesql status` — the store's state at a glance.
#[napi]
pub fn status(db_path: Option<String>) -> Result<String> {
    let (db, p) = open_db(db_path)?;
    let r = nesql::cmd::status::run(&db, &p);
    Ok(r.human)
}

/// `nesql root verify` — the stored state-root record vs a fresh recomputation.
#[napi]
pub fn root_verify(db_path: Option<String>, at: Option<f64>) -> Result<String> {
    let (db, _p) = open_db(db_path)?;
    let r = nesql::cmd::root::verify(&db, at.map(|v| v as u64));
    Ok(r.human)
}
