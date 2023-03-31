// Copyright (c) 2022-2023 The MobileCoin Foundation

//! SGX Attributes types

use crate::impl_newtype;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use bitflags::bitflags;
use core::fmt::{Display, Formatter};
use mc_sgx_core_sys_types::{
    sgx_attributes_t, sgx_misc_attribute_t, sgx_misc_select_t, SGX_CONFIGID_SIZE,
};

/// Attributes of the enclave
#[repr(transparent)]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Attributes(sgx_attributes_t);
impl_newtype! {
    Attributes, sgx_attributes_t;
}

impl Attributes {
    /// Set the `flags` for the attributes
    ///
    /// # Arguments
    ///
    /// * `flags` - The flags to be set in the attributes
    pub fn set_flags(mut self, flags: u64) -> Self {
        self.0.flags = flags;
        self
    }

    /// Set the extended features request mask (xfrm)
    ///
    /// # Arguments
    ///
    /// * `features_mask` - The mask to be set to the `xfrm` in the attributes
    pub fn set_extended_features_mask(mut self, features_mask: u64) -> Self {
        self.0.xfrm = features_mask;
        self
    }
}

impl Display for Attributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut display_string = "The following flags are set: ".to_string();
        let mut flags = Vec::new();
        if self.0.flags & AttributeFlags::SGX_FLAGS_INITTED {
            flags.push("SGX_FLAGS_INITTED");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_DEBUG {
            flags.push("SGX_FLAGS_DEBUG");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_MODE64BIT {
            flags.push("SGX_FLAGS_MODE64BIT");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_PROVISION_KEY {
            flags.push("SGX_FLAGS_PROVISION_KEY");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_EINITTOKEN_KEY {
            flags.push("SGX_FLAGS_EINITTOKEN_KEY");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_KSS {
            flags.push("SGX_FLAGS_KSS");
        }
        if self.0.flags & AttributeFlags::SGX_FLAGS_NON_CHECK_BITS {
            flags.push("SGX_FLAGS_NON_CHECK_BITS");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_LEGACY {
            flags.push("SGX_XFRM_LEGACY");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_AVX {
            flags.push("SGX_XFRM_AVX");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_AVX512 {
            flags.push("SGX_XFRM_AVX512");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_MPX {
            flags.push("SGX_XFRM_MPX");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_PKRU {
            flags.push("SGX_XFRM_PKRU");
        }
        if self.0.flags & AttributeFlags::SGX_XFRM_AMX {
            flags.push("SGX_XFRM_AMX");
        }
        let flags = flags.join(",");
        display_string.push_str(&flags);

        write!(f, display_string)
    }
}

bitflags! {
    #[derive(Deserialize, Serialize)]
    pub struct AttributeFlags: u64 {
        /// If set, then the enclave is initialized
        const SGX_FLAGS_INITTED = 0x0000000000000001;

        /// If set, then the enclave is debug
        const SGX_FLAGS_DEBUG = 0x0000000000000002;

        /// If set, then the enclave is 64 bit
        const SGX_FLAGS_MODE64BIT = 0x0000000000000004;

        /// set, then the enclave has access to provision key
        const SGX_FLAGS_PROVISION_KEY = 0x0000000000000010;

        /// If set, then the enclave has access to EINITTOKEN key
        const SGX_FLAGS_EINITTOKEN_KEY = 0x0000000000000020;

        /// If set enclave uses KSS
        const SGX_FLAGS_KSS = 0x0000000000000080;

        /// BIT[55-48] will not be checked */
        const SGX_FLAGS_NON_CHECK_BITS = 0x00FF000000000000;

        /// Legacy XFRM which includes the basic feature bits required by SGX, x87 state(0x01) and SSE state(0x02)
        const SGX_XFRM_LEGACY = 0x0000000000000003;

        /// AVX XFRM which includes AVX state(0x04) and SSE state(0x02) required by AVX
        const SGX_XFRM_AVX = 0x0000000000000006;

        /// AVX-512 XFRM
        const SGX_XFRM_AVX512 = 0x00000000000000E6;

        /// MPX XFRM - not supported
        const SGX_XFRM_MPX = 0x0000000000000018;

        /// PKRU state
        const SGX_XFRM_PKRU = 0x0000000000000200;

        /// AMX XFRM, including XTILEDATA(0x40000) and XTILECFG(0x20000)
        const SGX_XFRM_AMX = 0x0000000000060000;
    }
}

/// Miscellaneous select bits for target enclave. Reserved for future extension.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct MiscellaneousSelect(sgx_misc_select_t);

impl_newtype! {
    MiscellaneousSelect, sgx_misc_select_t;
}

/// Miscellaneous attributes and select bits for target enclave.
#[repr(transparent)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MiscellaneousAttribute(sgx_misc_attribute_t);

impl_newtype! {
    MiscellaneousAttribute, sgx_misc_attribute_t;
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use yare::parameterized;

    #[test]
    fn sgx_attributes_to_attributes() {
        let sgx_attributes = sgx_attributes_t { flags: 1, xfrm: 2 };
        let attributes: Attributes = sgx_attributes.into();
        assert_eq!(attributes.0, sgx_attributes);
    }

    #[test]
    fn attributes_to_sgx_attributes() {
        let attributes = Attributes(sgx_attributes_t { flags: 9, xfrm: 12 });
        let sgx_attributes: sgx_attributes_t = attributes.into();
        assert_eq!(sgx_attributes, sgx_attributes_t { flags: 9, xfrm: 12 });
    }

    #[parameterized(
    three_five = { 3, 5 },
    four_nine = { 4, 9 },
    )]
    fn attributes_builder(flags: u64, transform: u64) {
        let attributes = Attributes::default()
            .set_flags(flags)
            .set_extended_features_mask(transform);
        assert_eq!(attributes.0.flags, flags);
        assert_eq!(attributes.0.xfrm, transform);
    }
}
