// Type Objects & Constructors required by PEP 249
//
// Many databases need to have the input in a particular format
// for binding to an operation's input parameters. For example,
// if an input is destined for a DATE column, then it must be
// bound to the database in a particular string format. Similar
// problems exist for "Row ID" columns or large binary items
// (e.g. blobs or RAW columns). This presents problems for Python
// since the parameters to the .execute*() method are untyped.
// When the database module sees a Python string object, it doesn't
// know if it should be bound as a simple CHAR column, as a raw
// BINARY item, or as a DATE.
//
// To overcome this problem, a module must provide the constructors
// defined below to create objects that can hold special values. When
// passed to the cursor methods, the module can then detect the proper
// type of the input parameter and bind it accordingly.
//
// A Cursor Object's description attribute returns information about
// each of the result columns of a query. The type_code must compare
// equal to one of Type Objects defined below. Type Objects may be equal
// to more than one type code (e.g. DATETIME could be equal to the type
// codes for date, time and timestamp columns; see the implementations
// below for details).
//
// The module exports the following constructors and singletons:

// <editor-fold desc="// Type Objects ...">

// Sample implementation of Unix ticks based constructors
// for date/time delegating work to the generic constructors
//
// import time
//
// def DateFromTicks(ticks):
//     return Date(*time.localtime(ticks)[:3])
//
// def TimeFromTicks(ticks):
//     return Time(*time.localtime(ticks)[3:6])
//
// def TimestampFromTicks(ticks):
//     return Timestamp(*time.localtime(ticks)[:6])

// Required Type Objects
// STRING type
// BINARY type
// NUMBER type
// DATETIME type
// ROWID type
// SQL NULL values are represented by the Python None singleton on input and output.

// </editor-fold desc="// Type Objects ...">
