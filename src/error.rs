//! Errors for the soudning-base crate.

error_chain!{

    errors{

        /// A logical error discovered during sounding validation.
        ValidationError(msg: String) {
            display("Error validating sounding: {}", msg)
        }
    }
}
