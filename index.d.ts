// neSQL — PostgreSQL's grammar, NEDB's memory.
/** The PostgreSQL release this package's vendored grammar is taken from. */
export declare const VENDORED_POSTGRES: string;
export declare const VERSION: string;
/** Where the working PostgreSQL wire endpoint lives today. */
export declare const ENGINE: string;
/** Is this a usable query engine yet? No — and it says so rather than pretending. */
export declare function isRelease(): boolean;
