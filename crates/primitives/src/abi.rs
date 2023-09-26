//! Eth ABI helpers.

use crate::constants::SELECTOR_LEN;
use alloy_sol_types::{GenericContractError, SolInterface};

/// Returns the revert reason from the given output data, if it's an abi encoded String. Returns
/// `None` if the output is not long enough to contain a function selector or the content is not a
/// valid abi encoded String.
///
/// **Note:** it's assumed the `out` buffer starts with the call's signature
pub fn decode_revert_reason(out: &[u8]) -> Option<String> {
    // Ensure the output data is long enough to contain a function selector.
    if out.len() < SELECTOR_LEN {
        return None
    }

    // Try to decode as a generic contract error.
    if let Ok(error) = GenericContractError::decode(out, true) {
        return Some(error.to_string())
    }

    // If that fails, try to decode as a regular string.
    if let Ok(decoded_string) = std::str::from_utf8(out) {
        return Some(decoded_string.to_string())
    }

    // If both attempts fail, return None.
    None
}
