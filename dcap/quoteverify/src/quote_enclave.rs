// Copyright (c) 2022 MobileCoin Foundation

//! Provides functionality for interacting with the quoting enclaves.  Both the
//! QE(quoting enclave) and the PCE(provisioning certificate enclave)

use mc_sgx_dcap_quoteverify_types::Path as QvPath;
use mc_sgx_dcap_types::{Quote3Error, RequestPolicy};
use mc_sgx_util::ResultInto;
use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

/// Set path for QVE(Quote verification enclave) or QPL(Quote provider library)
///
/// This allows one to override the path that will be searched for each
/// `path_type`.  When this isn't called then the local path and dlopen search
/// path will be utilized.
///
/// Returns [`Quote3Error`] when
/// * `path` does not point to a file
/// * `path` is longer than 259 (bytes)
/// * `path` contains a null (0) byte.
///
/// # Arguments
/// * `path_type` - Which path to set
/// * `path` - The path value to use.  This is the full path to a file.
pub fn set_path<P: AsRef<Path>>(path_type: QvPath, path: P) -> Result<(), Quote3Error> {
    let c_path = CString::new(path.as_ref().as_os_str().as_bytes())
        .map_err(|_| Quote3Error::InvalidParameter)?;
    unsafe { mc_sgx_dcap_quoteverify_sys::sgx_qv_set_path(path_type.into(), c_path.as_ptr()) }
        .into_result()
}

/// Set the load policy
///
/// # Arguments
/// * `policy` - The policy to use for loading quoting enclaves
pub fn load_policy(policy: RequestPolicy) -> Result<(), Quote3Error> {
    unsafe { mc_sgx_dcap_quoteverify_sys::sgx_qv_set_enclave_load_policy(policy.into()) }
        .into_result()
}

#[cfg(test)]
mod test {
    use super::*;
    use mc_sgx_dcap_quoteverify_types::Path::{QuoteProviderLibrary, QuoteVerificationEnclave};
    use std::fs;
    use tempfile::tempdir;
    use yare::parameterized;

    #[test]
    fn qve_path_succeeds() {
        let dir = tempdir().unwrap();
        let file_name = dir.path().join("fake.txt");
        fs::write(&file_name, "stuff").unwrap();
        assert!(set_path(QuoteVerificationEnclave, file_name).is_ok());
    }

    #[test]
    fn qpl_path_succeeds() {
        let dir = tempdir().unwrap();
        let file_name = dir.path().join("fake.txt");
        fs::write(&file_name, "stuff").unwrap();
        assert!(set_path(QuoteProviderLibrary, file_name).is_ok());
    }

    #[test]
    fn path_as_directory_fails() {
        let dir = tempdir().unwrap();
        assert!(set_path(QuoteVerificationEnclave, dir.path()).is_err());
    }

    #[test]
    fn path_with_0_byte_fails_in_c_string() {
        let dir = tempdir().unwrap();
        let file_name = dir.path().join("fake\0.txt");
        // fs::write() will fail to create the file with a null byte in the path
        // so we pass the path as non existent to `set_path`.
        assert!(set_path(QuoteVerificationEnclave, file_name).is_err());
    }

    #[test]
    fn path_length_at_max_ok() {
        const MAX_PATH: usize = 259;
        let dir = tempdir().unwrap();
        let mut dir_length = dir.path().as_os_str().as_bytes().len();
        dir_length += 1; // for the joining "/"

        let long_name = str::repeat("a", MAX_PATH - dir_length);
        let file_name = dir.path().join(long_name);
        fs::write(&file_name, "stuff").unwrap();

        assert!(set_path(QuoteProviderLibrary, file_name).is_ok());
    }

    #[test]
    fn path_length_exceeded() {
        const MAX_PATH: usize = 259;
        let dir = tempdir().unwrap();
        let mut dir_length = dir.path().as_os_str().as_bytes().len();
        dir_length += 1; // for the joining "/"

        let long_name = str::repeat("a", (MAX_PATH + 1) - dir_length);
        let file_name = dir.path().join(long_name);
        fs::write(&file_name, "stuff").unwrap();

        assert!(set_path(QuoteProviderLibrary, file_name).is_err());
    }

    #[parameterized(
    persistent = { RequestPolicy::Persistent },
    ephemeral = { RequestPolicy::Ephemeral },
    )]
    fn load_policy_succeeds(policy: RequestPolicy) {
        assert!(load_policy(policy).is_ok());
    }
}