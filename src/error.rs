#![allow(missing_docs, unused_doc_comment)]
//! Error types for the sounding-base crate.

error_chain!{

    errors{

        /// A logical error discovered during sounding validation.
        ValidationError(msg: String) {
            display("Error validating sounding: {}", msg)
        }
    }
}
