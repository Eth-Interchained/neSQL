// SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
// SPDX-License-Identifier: BUSL-1.1
// neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)

//! # neSQL — PostgreSQL's grammar, NEDB's memory
//!
//! SQL you already know, over a database that never forgets and can prove it.
//!
//! **This crate is a reserved name and a statement of intent.** The grammar is
//! vendored (PostgreSQL 17.4 `gram.y`, licence intact) and the executor
//! foundations — joins, subqueries, set operations, `array_agg(x ORDER BY y)`,
//! derived tables — already ship inside [`nedb-engine`]. What lands here is the
//! front-end that lets them be reached by ordinary SQL rather than by a
//! translated dialect.
//!
//! Until then, the working PostgreSQL wire endpoint lives in `nedb-engine`:
//! `nedbd --pg-port 5433` answers `psql`, SQLAlchemy, asyncpg and
//! node-postgres today.
//!
//! [`nedb-engine`]: https://crates.io/crates/nedb-engine

/// The PostgreSQL release this crate's vendored grammar is taken from.
pub const VENDORED_POSTGRES: &str = "17.4";

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Where the real thing runs today.
pub const ENGINE: &str = "https://github.com/Eth-Interchained/nedb";

/// Is this a usable query engine yet? No — and it says so rather than pretending.
pub const fn is_release() -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_does_not_overstate_itself() {
        assert!(!super::is_release());
        assert_eq!(super::VENDORED_POSTGRES, "17.4");
    }
}
