# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
# neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)
"""neSQL — PostgreSQL's grammar, NEDB's memory.

The neSQL language: PostgreSQL's real grammar, vendored from 17.4 with its
licence intact, extended with the temporal, causal and full-text clauses a
permanent, hash-chained store can answer. This package carries the compiled
command surface as ``nesql._native`` (``nesql.__has_native__`` reports whether
it loaded) — the same one implementation the ``nesql`` CLI binary uses.

The engine beneath it ships as ``nedb-engine`` — one pip install carries the
daemon, the Python engine and this CLI surface together.
"""

from __future__ import annotations

__version__ = "8.0.1"

#: The PostgreSQL release the vendored grammar is taken from.
VENDORED_POSTGRES = "17.4"

#: Where the engine (daemon, runtime) ships.
ENGINE = "https://github.com/Eth-Interchained/nedb"

try:
    from . import _native  # type: ignore
    from ._native import Nesql  # type: ignore
    __has_native__ = True
    _N = _native
except ImportError:  # pragma: no cover — universal wheel / non-native platforms
    __has_native__ = False
    _N = None
    Nesql = None


def query(q: str, db_path: str | None = None, forced: str | None = None) -> str:
    """Run a neSQL query (SQL or the FROM-form) — the native command surface.

    Returns the same JSON object ``nesql --json query`` prints: ok/exit/status/
    dialect/rows. Requires the native core (a platform wheel install).
    """
    if _N is None:
        raise RuntimeError(
            "nesql native core not available — install a platform wheel of nesql "
            "(or nedb-engine, which carries the same CLI binary)")
    return _N.query(q, db_path, forced)


def version() -> str:
    """The CLI's version block — nesql / engine / grammar digest."""
    if _N is None:
        import subprocess
        out = subprocess.run(["nesql", "version"], capture_output=True, text=True)
        return out.stdout or "nesql ?\n(no native core; CLI binary not found)"
    return _N.version()


__all__ = ["VENDORED_POSTGRES", "__version__", "ENGINE", "query", "version",
           "Nesql", "__has_native__"]
