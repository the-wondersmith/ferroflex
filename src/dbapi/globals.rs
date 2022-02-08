// Module-level "global" constants required by PEP 249

// <editor-fold desc="// Globals ...">

/// String constant stating the supported DB API level.
/// Per PEP 249, only the strings "1.0" and "2.0" are allowed.
pub const APILEVEL: &str = "2.0";

/// Integer constant stating the level of thread safety the interface supports.
/// Possible values are:
///   - 0: Threads may not share the module
///   - 1: Threads may share the module, but not connections
///   - 2: Threads may share the module and connections
///   - 3: Threads may share the module, connections and cursors
pub const THREADSAFETY: usize = 1;

/// String constant stating the type of parameter marker formatting expected by the interface.
/// Possible values are:
///  - "qmark": Question mark style, e.g. ...WHERE name=?
///  - "numeric": Numeric, positional style, e.g. ...WHERE name=:1
///  - "named": Named style, e.g. ...WHERE name=:name
///  - "format": ANSI C printf format codes, e.g. ...WHERE name=%s
///  - "pyformat": Python extended format codes, e.g. ...WHERE name=%(name)s
pub const PARAMSTYLE: &str = "qmark";

// </editor-fold desc="// Globals ...">
