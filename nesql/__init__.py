# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
# neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)
"""neSQL — PostgreSQL's grammar, NEDB's memory.

This package is the PyPI mirror of the neSQL language reference: the vendored
PostgreSQL grammar facts, the NEDB clause extensions, and where the working
engine and CLI live.

The WORKING engine and the `nesql` CLI ship inside nedb-engine — one install
carries both::

    pip install nedb-engine
    nesql --db ./store query "SELECT * FROM orders AS OF SYSTEM TIME 42"
    nedbd --pg-port 5433        # psql / SQLAlchemy / asyncpg / node-postgres

This repository (Eth-Interchained/neSQL) carries the language itself: the
vendored PostgreSQL grammar with its licence intact, the real `nesql` CLI
crate (rust/), the NQL grammar reference, and the NEDB specs.
"""

#: The PostgreSQL release this package's vendored grammar is taken from.
VENDORED_POSTGRES = "17.4"
__version__ = "8.0.0"
#: Where the working engine + CLI live (one pip install carries both).
ENGINE = "https://github.com/Eth-Interchained/nedb"

__all__ = ["VENDORED_POSTGRES", "__version__", "ENGINE"]
