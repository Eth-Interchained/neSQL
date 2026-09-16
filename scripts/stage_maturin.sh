#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
#
# Stage everything the maturin native wheel needs into crates/nesql-py/.
# Same practice as the engine's stage_maturin.sh: maturin only packages files
# inside its project root, so the python package + readme are staged in.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dest="$root/crates/nesql-py"
rm -rf "$dest/python"
mkdir -p "$dest/python"
cp -r "$root/nesql" "$dest/python/nesql"
cp "$root/README.md" "$dest/README.md" 2>/dev/null || true
cp "$root/LICENSE" "$dest/LICENSE" 2>/dev/null || true
cp "$root/COPYING-APACHE-2.0.txt" "$dest/" 2>/dev/null || true
echo "staged: $dest/python/nesql"
