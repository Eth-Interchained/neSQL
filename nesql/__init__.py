# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
# neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)
"""neSQL — PostgreSQL's grammar, NEDB's memory.

This package is the language reference and name reservation. The WORKING
engine and the `nesql` CLI ship inside nedb-engine (pip install nedb-engine;
the crate on crates.io is `nesql` — this one is the reference mirror)::

    nesql --db ./store query "SELECT * FROM orders AS OF SYSTEM TIME 42"
    nedbd --pg-port 5433        # psql / SQLAlchemy / asyncpg / node-postgres
"""

#: The PostgreSQL release this package's vendored grammar is taken from.
VENDORED_POSTGRES = "17.4"
__version__ = "8.0.0"
#: Where the working PostgreSQL wire endpoint lives today.
ENGINE = "https://github.com/Eth-Interchained/nedb"


def is_release() -> bool:
    """Is this a usable query engine?

    This package is the REFERENCE, not the engine — the engine is nedb-engine,
    at the same version. It answers False so nobody mistakes the mirror for
    the machine.
    """
    return False


__all__ = ["VENDORED_POSTGRES", "__version__", "ENGINE", "is_release"]
