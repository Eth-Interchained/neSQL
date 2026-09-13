<div align="center">

# neSQL

**PostgreSQL's grammar. NEDB's memory.**

*The SQL you already write — over a database that never forgets, and can prove it.*

[![PyPI](https://img.shields.io/pypi/v/nesql?label=PyPI&color=6366f1)](https://pypi.org/project/nesql/)
[![npm](https://img.shields.io/npm/v/nesql-engine?label=npm&color=00d4ff)](https://www.npmjs.com/package/nesql-engine)
[![crates.io](https://img.shields.io/crates/v/nesql?label=crates.io&color=f97316)](https://crates.io/crates/nesql)
[![status](https://img.shields.io/badge/status-pre--release-a855f7)](https://github.com/Eth-Interchained/neSQL)
[![grammar](https://img.shields.io/badge/grammar-PostgreSQL%2017.4-336791)](vendor/postgresql/COPYRIGHT)

**[nedb-engine](https://github.com/Eth-Interchained/nedb)** · **[Studio](https://studio.interchained.org)**

</div>

---

## Nobody should have to learn a query language to use a database

That sentence cost us a query language.

NEDB shipped with **NQL** — a FROM-first language built around the things NEDB can do that
nothing else can: reach any point in history, ask what was *believed true* as of a date,
walk the causal chain that produced a record. It works. It is also a tax: before you can
ask NEDB a question, you have to learn how to ask.

Meanwhile the engine grew a PostgreSQL wire-protocol endpoint, and the SQL arriving on it
was translated into NQL by **string surgery**. That is where every interesting bug lived.
A qualified column (`WHERE orders.status = 'paid'`) was rewritten by hand and silently
matched nothing. `HAVING n > 1` passed through verbatim and answered zero rows. `total * 2`
became a *field name* nothing was called, so the column came back blank. Not one of those
was an engine bug. Every one was a translation bug.

And translation had a ceiling it could never clear. NQL's grouped row carries the group key,
`count`, and exactly **one** named aggregate — so `SELECT sum(total), avg(total) GROUP BY status`
was not slow or degraded, it was *unrepresentable*. No amount of cleverness in the translator
fixes a row model.

**So we stopped translating.**

## What neSQL is

PostgreSQL's real grammar — `gram.y`, all 19,513 lines and 492 keywords of it, vendored from
**PostgreSQL 17.4** with its copyright notice intact — extended with the handful of clauses
NEDB needs to say what it can actually do.

```
vendor/postgresql/
  gram.y                   19,513 lines   the real parser, unmodified upstream
  kwlist.h                    492 keywords
  system_views.sql          1,377 lines   pg_catalog, Postgres's own definitions
  information_schema.sql    3,046 lines   the standard catalogue, verbatim
  COPYRIGHT                    23 lines   permissive; modification expressly granted
```

We are not reimplementing SQL from memory. We are starting from the definition every
other tool in the world was built against, and adding to it — the same road
CockroachDB, Materialize and RisingWave all took.

### The clauses we add, and why

PostgreSQL's grammar contains **no** temporal SQL — we checked, and `SYSTEM_TIME`,
`PERIOD` and `PORTION` appear exactly zero times in `gram.y`. Time travel was never
going to arrive for free. So it arrives deliberately:

| neSQL | what it answers | precedent |
| --- | --- | --- |
| `AS OF SYSTEM TIME <seq>` | the exact state at a point in history | CockroachDB |
| `VALID AS OF <time>` | what was *believed true* as of then | SQL:2011 application time |
| `TRACE <id>` | the causal chain that produced this record | `WITH RECURSIVE` |
| `LINK` / `TRAVERSE` | relationships, without a join table | graph SQL |
| `SEARCH` | full-text over document fields | `tsquery` |
| `_hash` `_seq` `_caused_by` | provenance, selectable like any column | — |

And what the vendored grammar hands us for free, which the old translator refused
by name: `WITH RECURSIVE`, window functions, `GROUPING SETS`, and `MERGE` — upsert,
which lands remarkably naturally on an append-only store.

## What this means if you already speak Postgres

You are done learning.

```sql
-- ordinary SQL. nothing to look up.
SELECT status, sum(total), avg(total)
  FROM orders
 WHERE region = 'eu'
 GROUP BY status
HAVING count(*) > 1;

-- the same query, ninety thousand writes ago
SELECT status, sum(total), avg(total)
  FROM orders AS OF SYSTEM TIME 412
 GROUP BY status;

-- why does this row say what it says?
TRACE 'order-8814';
```

The first query is the point. It is not a NEDB query. It is a query — and the audit
trail underneath it is free, permanent, and hash-verified, with no triggers, no shadow
tables, and no application code.

## Status: pre-release, and honest about it

This repository is the **grammar and the front-end**, being built in the open. It is
not installable yet, and the package on each registry is a reserved name, not a
product. When that changes it will change here first.

| | |
| --- | --- |
| ✅ | PostgreSQL 17.4 grammar + catalogue vendored, licence intact |
| ✅ | Executor foundations shipped in nedb-engine — joins, subqueries, set operations, `array_agg(x ORDER BY y)`, derived tables, `generate_series` |
| 🔨 | neSQL grammar delta — the temporal and causal clauses above |
| 🔨 | One IR: neSQL and NQL compiling to the same plan, so nothing translates |
| 🔨 | The executor pointed at user collections, not only the catalogue |
| 📋 | Cross-front-end parity assertions — the same question through both, same answer |

**Today, right now, in production:** the PostgreSQL endpoint in
[`nedb-engine`](https://github.com/Eth-Interchained/nedb) already answers `psql`,
SQLAlchemy (Core *and* ORM), asyncpg and node-postgres against a live store, with
`pg_catalog` and `information_schema` implemented as genuinely queryable relations.
neSQL is where that stops needing an asterisk.

## The names

```bash
pip install nesql                  # PyPI
cargo add nesql                    # crates.io
npm install nesql-engine           # npm  (or @interchained/nesql)
```

All three are **reserved placeholders** today — each one loads, reports the vendored
PostgreSQL release, and answers `is_release() == false`. None of them pretends to be a
driver, because a package that imports cleanly and then lies is worse than one that
isn't published yet.

On npm the bare name `nesql` is refused by the registry's typosquat guard — *"too
similar to existing packages mssql, mysql"* — which, given the company that puts us in,
we will take. `nesql-engine` is the unscoped name and `@interchained/nesql` is the
scoped one; both are ours.

## Licence

neSQL is **BUSL-1.1** — free in production under USD $1M annual revenue, converting
to Apache 2.0 automatically. See [`LICENSE`](LICENSE).

The vendored PostgreSQL sources under `vendor/postgresql/` remain under the
**PostgreSQL Licence**, reproduced verbatim and unmodified at
[`vendor/postgresql/COPYRIGHT`](vendor/postgresql/COPYRIGHT). Our thanks to the
PostgreSQL Global Development Group — thirty years of grammar we did not have to
guess at.

---

<div align="center">

**© INTERCHAINED LLC** · built with **Vex** (Claude Opus 5)

*Interchained builds tools that outlive the demo.*

</div>
