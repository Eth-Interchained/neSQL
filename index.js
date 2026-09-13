// SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
// SPDX-License-Identifier: BUSL-1.1
// neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)

// neSQL — PostgreSQL's grammar, NEDB's memory.
//
// This package is a reserved name and a statement of intent, and it reports
// that rather than pretending to be a driver. The working PostgreSQL wire
// endpoint ships TODAY in nedb-engine: `nedbd --pg-port 5433` is answered by
// node-postgres, psql, SQLAlchemy and asyncpg against a live store.

"use strict";

const VENDORED_POSTGRES = "17.4";
const VERSION = require("./package.json").version;
const ENGINE = "https://github.com/Eth-Interchained/nedb";

/** Is this a usable query engine yet? No — and it says so rather than pretending. */
function isRelease() {
  return false;
}

module.exports = { VENDORED_POSTGRES, VERSION, ENGINE, isRelease };
