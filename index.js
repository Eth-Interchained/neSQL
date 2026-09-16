// SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
// SPDX-License-Identifier: BUSL-1.1
// neSQL — PostgreSQL's grammar, NEDB's memory.
//
// npm mirror of the neSQL language reference. The WORKING engine and the
// `nesql` CLI ship inside nedb-engine on npm — one install carries both:
//
//   npm install nedb-engine
//   const { nesql } = require("nedb-engine");   // CLI entry also on PATH
//
// This repository (Eth-Interchained/neSQL) carries the language itself:
// the vendored PostgreSQL grammar (licence intact), the real nesql CLI
// crate (rust/), the NQL grammar reference, and the NEDB specs.

"use strict";

const VENDORED_POSTGRES = "17.4";
const VERSION = require("./package.json").version;
const ENGINE = "https://github.com/Eth-Interchained/nedb";

module.exports = { VENDORED_POSTGRES, VERSION, ENGINE };
