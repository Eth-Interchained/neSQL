#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
# neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)
#
# Regenerate reference/ from a checkout of the engine.
#
#   ./scripts/sync-reference.sh ../nedb
#
# Everything under reference/ is a COPY of something whose source of truth
# lives in Eth-Interchained/nedb. That is a deliberate trade: onlookers get to
# read the CLI and both halves of the grammar in the repo that bears the
# language's name, at the cost of the copy being able to fall behind.
#
# This script is what keeps that cost payable. The alternative -- hand-editing
# files under reference/ -- produces exactly the failure this whole directory
# was created in response to: a published grammar that quietly stopped
# describing the shipped binary. v6.0.0 printed "diff, tag, branch, merge
# reserved; not yet wired" for four verbs that had worked since #143, and its
# grammar digest was byte-identical to v5.0.1's because the text had never
# been edited. A copy nobody can regenerate is a copy that will say the same
# kind of thing.
#
# So: reference/ is generated, this is the generator, and the digest in
# grammar.json is the thing to compare when you want to know whether the copy
# is current.

set -euo pipefail

ENGINE="${1:-}"
if [ -z "$ENGINE" ]; then
  echo "usage: $0 <path-to-nedb-checkout>" >&2
  exit 2
fi
if [ ! -f "$ENGINE/rust/nedb-v2/src/nql.rs" ]; then
  # Named specifically rather than "not found": the usual mistake is pointing
  # this at the neSQL repo itself.
  echo "error: $ENGINE does not look like an Eth-Interchained/nedb checkout" >&2
  echo "       (expected rust/nedb-v2/src/nql.rs beneath it)" >&2
  exit 2
fi

HERE="$(cd "$(dirname "$0")/.." && pwd)"
ENGINE="$(cd "$ENGINE" && pwd)"
COMMIT="$(git -C "$ENGINE" rev-parse --short=8 HEAD)"
VERSION="$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$ENGINE/rust/nesql-cli/Cargo.toml" | head -1)"

echo "engine  $ENGINE"
echo "commit  $COMMIT"
echo "version $VERSION"

# ── the CLI source ──────────────────────────────────────────────────────────
rm -rf "$HERE/reference/nesql-cli"
mkdir -p "$HERE/reference/nesql-cli"
cp -r "$ENGINE/rust/nesql-cli/src" \
      "$ENGINE/rust/nesql-cli/tests" \
      "$ENGINE/rust/nesql-cli/Cargo.toml" \
      "$HERE/reference/nesql-cli/"

# ── the command surface, from the binary itself ─────────────────────────────
# Built, not transcribed. `nesql grammar` is the command whose entire job is
# publishing this, so asking the binary is the only answer that cannot drift
# from the binary.
mkdir -p "$HERE/reference/neql"
( cd "$ENGINE/rust" && cargo build -q -p nesql-cli )
BIN="$ENGINE/rust/target/debug/nesql"
"$BIN" grammar        > "$HERE/reference/neql/grammar.txt"
"$BIN" grammar --json > "$HERE/reference/neql/grammar.json"

# ── NQL's own clause grammar ────────────────────────────────────────────────
python3 - "$ENGINE" "$COMMIT" "$HERE" <<'PY'
import sys, os
engine, commit, here = sys.argv[1], sys.argv[2], sys.argv[3]
rel = "rust/nedb-v2/src/nql.rs"
doc = []
for line in open(os.path.join(engine, rel)).read().splitlines():
    if line.startswith("//!"):
        # Exactly ONE space of the comment marker, never lstrip(): the grammar
        # is indentation-structured, and stripping it flattens
        #     FROM coll
        #       [AS OF seq]
        # into a list of unrelated lines. The first version of this script did
        # that and produced a "reference" that lost the nesting.
        doc.append(line[4:] if line.startswith("//! ") else line[3:])
    elif doc and not line.startswith("//"):
        break

header = f"""NQL — the temporal/causal half of neQL
=======================================

GENERATED. Do not edit by hand; run scripts/sync-reference.sh.

  source   {rel}
  commit   {commit}  (Eth-Interchained/nedb)

This is the grammar as the RUST engine documents it — the implementation that
backs nedbd, the napi addon and the PyO3 wheel. NEDB also carries an
independent Python reference implementation (python/nedb/query.py); the two
are held to identical answers by cross-engine parity tests in the engine
repository, but this file is extracted from the Rust one and says so rather
than claiming to describe both.

NQL is one half of neQL. The other half is PostgreSQL's own grammar, vendored
under vendor/postgresql/. Which half a statement is read as is decided by its
leading keyword — see reference/neql/grammar.txt.

-----------------------------------------------------------------------------

"""
out = os.path.join(here, "reference/neql/nql-grammar.txt")
open(out, "w").write(header + "\n".join(doc).rstrip() + "\n")
print("  nql-grammar.txt %d bytes" % os.path.getsize(out))
PY

# ── provenance, in one machine-readable place ───────────────────────────────
python3 - "$COMMIT" "$VERSION" "$HERE" <<'PY'
import sys, json, os, hashlib
commit, version, here = sys.argv[1], sys.argv[2], sys.argv[3]
g = json.load(open(os.path.join(here, "reference/neql/grammar.json")))
files = []
for root, _, names in os.walk(os.path.join(here, "reference")):
    for n in sorted(names):
        p = os.path.join(root, n)
        if os.path.basename(p) == "SOURCE.json":
            continue
        files.append(os.path.relpath(p, here))
open(os.path.join(here, "reference/SOURCE.json"), "w").write(json.dumps({
    "_comment": "Generated by scripts/sync-reference.sh. Everything under "
                "reference/ is a copy; this records what it was copied from.",
    "engine_repo": "https://github.com/Eth-Interchained/nedb",
    "engine_commit": commit,
    "cli_version": version,
    "grammar_version": g["version"],
    "grammar_digest": g["digest"],
    "files": sorted(files),
}, indent=2) + "\n")
print("  SOURCE.json     %s @ %s" % (version, commit))
PY

echo "done"
