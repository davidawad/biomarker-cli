//! Health-data interchange: FHIR R4 and HL7 v2 in, FHIR R4 out.
//!
//! Everything read from outside becomes a [`candidate::ResultCandidate`] first
//! (`fhir.rs`, `hl7v2.rs`), then one pipeline (`pipeline.rs`) maps it to a
//! marker, converts the unit, dedupes it against what is already stored and
//! writes it. Lab-supplied reference ranges are kept per result
//! (`lab_result_meta`) and never mixed into the cited catalog ranges.

pub mod candidate;
pub mod export;
pub mod fhir;
pub mod hl7v2;
pub mod import;
pub mod loinc;
pub mod pipeline;
pub mod schema;
pub mod ucum;
