//! The Common Electric Power Transmission System Model (CTM) is an intuitive, extensible,
//! language-agnostic, and range-validating spefication of electric power network components'
//! parameter names and units, and the relation between components, intended for use by the
//! research community developing new computational methods for power systems operations and
//! simulation.
//!
//! This standard data structure in CTM makes it easy to work in multiple power systems domains
//! (e.g., economic operation, reliability assessment, electricity markets, stability assessment)
//! without requiring conversions between use-case-specific file formats and data structures
//! with information loss in the process.

#![allow(
    rustdoc::broken_intra_doc_links,
    irrefutable_let_patterns,
    dead_code,
    elided_named_lifetimes
)]
// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
#![allow(rustdoc::bare_urls)]
mod ctm;

pub use ctm::*;
