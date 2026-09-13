# SPDX-FileCopyrightText: 2026 INTERCHAINED LLC
# SPDX-License-Identifier: BUSL-1.1
# neSQL · © 2026 INTERCHAINED LLC × Eth-Interchained × Vex (Claude Opus 5)
"""neSQL — PostgreSQL's grammar, NEDB's memory.

This package is a reserved name and a statement of intent, and it reports that
rather than pretending to be a driver. The working PostgreSQL wire endpoint
ships TODAY in nedb-engine::

    nedbd --pg-port 5433

answered by psycopg2, SQLAlchemy (Core and ORM), asyncpg and node-postgres
against a live, tamper-evident store.
"""

#: The PostgreSQL release this package's vendored grammar is taken from.
VENDORED_POSTGRES = "17.4"
__version__ = "0.0.1"
#: Where the working PostgreSQL wire endpoint lives today.
ENGINE = "https://github.com/Eth-Interchained/nedb"


def is_release() -> bool:
    """Is this a usable query engine yet? No -- and it says so rather than pretending."""
    return False


__all__ = ["VENDORED_POSTGRES", "__version__", "ENGINE", "is_release"]
