use pgrx::prelude::*;

use tiktoken_rs::encoding_for_model;
use tiktoken_rs::r50k_base;
use tiktoken_rs::cl100k_base;
use tiktoken_rs::p50k_base;
use tiktoken_rs::p50k_edit;

pgrx::pg_module_magic!();

// encode to the array of tokens using given encoding/model
//
// encoding_selector could be the name of the encoding
// (cl100k_base, p50k_base, ...)
// or designated model (text-davinci-003, gpt-3.5-turbo, ...)
fn encode_with_model(encoding_selector: &str, text: &str) -> Vec<usize> {
    let encoder_name = encoding_for_model(encoding_selector)
        .unwrap_or(encoding_selector);

    let encoder = match encoder_name {
        "cl100k_base" => cl100k_base(),
        "r50k_base" | "gpt2" => r50k_base(),
        "p50k_base" => p50k_base(),
        "p50k_edit" => p50k_edit(),
        _ => error!("'{encoding_selector}': unknown model or encoder")
    }.unwrap();

    encoder.encode_with_special_tokens(text)
}

#[pg_extern]
fn tiktoken_encode(encoding_selector: &str, text: &str) -> Vec<i64> {
    encode_with_model(encoding_selector, text)
        .iter()
        .map(|&x| x.try_into().unwrap())
        .collect()
}

#[pg_extern]
fn tiktoken_count(encoding_selector: &str, text: &str) -> i64 {
    encode_with_model(encoding_selector, text)
        .len()
        .try_into()
        .unwrap()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_tiktoken_encode() {
        // check various encoders
        let str = "This is a test         with a lot of spaces<|endoftext|>";
        assert_eq!(
            crate::tiktoken_encode("p50k_base", str),
            vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("p50k_edit", str),
            vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("r50k_base", str),
            vec![1212, 318, 257, 1332, 220, 220, 220, 220, 220, 220, 220, 220, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("cl100k_base", str),
            vec![2028, 374, 264, 1296, 260, 449, 264, 2763, 315, 12908, 100257]
        );

        // check model aliases
        let str = "A long time ago in a galaxy far, far away";
        assert_eq!(
            crate::tiktoken_encode("text-davinci-002", str),
            crate::tiktoken_encode("p50k_base", str)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt-3.5-turbo", str),
            crate::tiktoken_encode("cl100k_base", str)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt2", str),
            crate::tiktoken_encode("r50k_base", str)
        );
        assert_eq!(
            crate::tiktoken_encode("code-davinci-edit-001", str),
            crate::tiktoken_encode("p50k_edit", str)
        );
    }

    #[pg_test]
    fn test_tiktoken_count() {
        let str = "A long time ago in a galaxy far, far away";
        assert_eq!(crate::tiktoken_count("p50k_base", str), 11);
        assert_eq!(crate::tiktoken_count("cl100k_base", str), 11);
        assert_eq!(crate::tiktoken_count("r50k_base", str), 11);
        assert_eq!(crate::tiktoken_count("p50k_edit", str), 11);
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
