// Rust representations of the various data structs in a DataFlex table file.

// Module Declarations
pub(crate) mod column;
pub(crate) mod database;
pub(crate) mod filelist;
pub(crate) mod index;
pub(crate) mod segment;
pub(crate) mod table;
pub(crate) mod tagfile;

// Third-Party Imports
use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

// Sub-Module "Exports"
pub use column::Column;
pub use database::DataFlexDB;
pub use filelist::{FileList, FileListEntry};
pub use index::Index;
pub use segment::FieldSegment;
pub use table::{DataFlexTable, Header, Row};
pub use tagfile::{TagCollection, TagFile};

// <editor-fold desc="// Component Registration ...">

/// Register the Rust code to be "exported" to Python
pub(crate) fn register_components(py: Python, ferroflex_module: &PyModule) -> PyResult<()> {
    // Create the `structs` sub-module
    let structs_module = PyModule::new(py, "ferroflex.structs")?;

    // Add the class objects to the module

    // Column
    structs_module.add_class::<Column>()?;

    // Database
    structs_module.add_class::<DataFlexDB>()?;

    // FileList
    structs_module.add_class::<FileList>()?;
    structs_module.add_class::<FileListEntry>()?;

    // Index
    structs_module.add_class::<Index>()?;

    // Segment
    structs_module.add_class::<FieldSegment>()?;

    // Table
    structs_module.add_class::<Row>()?;
    structs_module.add_class::<Header>()?;
    structs_module.add_class::<DataFlexTable>()?;

    // Tag File
    structs_module.add_class::<TagFile>()?;

    // Add the populated sub-module to the top-level `ferroflex` module
    ferroflex_module.add("structs", structs_module)?;

    // Return an OK
    Ok(())
}

// </editor-fold desc="// Component Registration ...">
