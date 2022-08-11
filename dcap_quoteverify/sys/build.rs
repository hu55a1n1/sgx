// Copyright (c) 2022 The MobileCoin Foundation
//! Builds the FFI function bindings for dcap quoteverify library of the Intel
//! SGX SDK

use cargo_emit::rustc_link_lib;

const DCAP_QL_FUNCTIONS: &[&str] = &[
    "sgx_qv_free_qve_identity",
    "sgx_qv_get_quote_supplemental_data_size",
    "sgx_qv_get_qve_identity",
    "sgx_qvl_get_quote_supplemental_data_size",
    "sgx_qvl_get_quote_supplemental_data_version",
    "sgx_qvl_verify_quote",
    "sgx_qv_set_enclave_load_policy",
    "sgx_qv_set_path",
    "sgx_qv_set_qpl_path",
    "sgx_qv_set_qve_path",
    "sgx_qv_verify_quote",
];

fn main() {
    rustc_link_lib!("dylib=sgx_dcap_quoteverify");

    let mut builder = mc_sgx_core_build::sgx_builder()
        .header("wrapper.h")
        .blocklist_type("*");

    for f in DCAP_QL_FUNCTIONS {
        builder = builder.allowlist_function(f);
    }

    let out_path = mc_sgx_core_build::build_output_path();
    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
