use alloc::string::String;
use base64::Engine;

/// Helper macro to create a `NamedArg` from an expression.
#[macro_export]
macro_rules! named_arg {
    ($name:expr, $value:expr) => {
        casper_types::NamedArg::new(
            alloc::string::ToString::to_string($name),
            casper_types::CLValue::from_t($value).unwrap(),
        )
    };

    ($value:expr) => {
        named_arg!(stringify!($value), $value)
    };
}

/// Create a non-Sync static variable accessible safely in a Casper contract.
#[macro_export]
macro_rules! st_non_sync_static {
    ($t:ty = $init:expr) => {
        // it makes no sense to use a Sync static with this macro, since
        // Sync values do not require unsafe to be read from static variables
        static_assertions::assert_not_impl_all!($t: Sync);

        #[cfg(onchain)]
        static mut GLOBAL: $t = $init;

        #[cfg(onchain)]
        pub(super) fn get() -> &'static $t {
            assert!(cfg!(target_arch = "wasm32"));

            // SAFETY: This is safe because we are in a Casper wasm32 environment, which
            // is single-threaded. Since only one thread can access this static at a time,
            // we can safely return a reference to it.
            unsafe { &GLOBAL }
        }

        #[cfg(not(onchain))]
        pub(super) fn get() -> &'static $t {
            #[allow(unused)]
            const _: $t = $init;
            panic!("cannot access non-Sync static in a non-wasm environment");
        }
    };
}

/// Helper macro to create a named key in the contract context, as well as accessors for it.
#[macro_export]
macro_rules! named_key {
    (@uref $name:ident, $t:ty) => {
        pub const NAME: &str = stringify!($name);

        mod cache {
            use super::*;
            use casper_types::URef;
            use once_cell::unsync::Lazy;

            $crate::st_non_sync_static!(Lazy<URef> = Lazy::new(|| load_uref()));

            pub fn uref() -> URef {
                **get()
            }
        }

        pub use cache::uref;

        /// Loads the URef from storage. Prefer to use `uref()` instead, which uses a cached value.
        pub fn load_uref() -> URef {
            let key = runtime::get_key(NAME).unwrap_or_revert_with(ApiError::MissingKey);
            key.into_uref()
                .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant)
        }

        pub fn put_uref(uref: URef) {
            runtime::put_key(NAME, uref.into());
        }

    };

    (dict $name:ident : $t:ty) => {
        pub mod $name {
            #[allow(unused)]
            use super::*;
            use casper_contract::contract_api::{runtime, storage};
            use casper_contract::unwrap_or_revert::UnwrapOrRevert;
            use casper_types::{ApiError, URef};

            $crate::named_key! { @uref $name, $t }

            pub fn try_read(key: &str) -> Option<$t> {
                storage::dictionary_get(uref(), key).unwrap_or_revert_with(ApiError::Deserialize)
            }

            pub fn read(key: &str) -> $t {
                try_read(key).unwrap_or_revert_with(ApiError::MissingKey)
            }

            pub fn write(key: &str, value: $t) {
                storage::dictionary_put(uref(), key, value);
            }

            pub fn remove(key: &str) {
                storage::dictionary_put(uref(), key, ());
            }

            pub fn init() {
                storage::new_dictionary(NAME).unwrap_or_revert_with(ApiError::User(u16::MAX));
            }
        }
    };

    (val $name:ident : $t:ty = $init:expr) => {
        pub mod $name {
            #[allow(unused)]
            use super::*;
            use casper_contract::contract_api::{runtime, storage};
            use casper_contract::unwrap_or_revert::UnwrapOrRevert;
            use casper_types::{ApiError, URef};

            $crate::named_key! { @uref $name, $t }

            pub fn try_read() -> Option<$t> {
                storage::read(uref()).unwrap_or_revert_with(ApiError::Deserialize)
            }

            pub fn read() -> $t {
                try_read().unwrap_or_revert_with(ApiError::MissingKey)
            }

            pub fn write(value: $t) {
                storage::write(uref(), value);
            }

        }
    };
}

/// Declare multiple named keys in one go.
///
/// Creates an extra function to initialize all the keys at once.
#[macro_export]
macro_rules! named_keys {
    (
        @init_all
        $v:ident
        val $name:ident : $t:ty = $init:expr
    ) => {
        ($name::NAME.to_string(), casper_types::Key::URef(casper_contract::contract_api::storage::new_uref($init)))
    };

    (
        @init_all
        $v:ident
        dict $name:ident : $t:ty
    ) => {
        ($name::NAME.to_string(), casper_types::Key::URef($crate::util::contract_api::new_dictionary_anon()))
    };

    (
        $init_all:ident:
        $( $k:ident $name:ident : $t:ty $(= $init:expr)? );* $(;)?
    ) => {
        $(
            $crate::named_key! { $k $name : $t $(= $init)? }
        )*

        pub fn $init_all() -> alloc::vec::Vec<(alloc::string::String, casper_types::Key)> {
            #[allow(unused)]
            use alloc::string::ToString;
            let mut v = alloc::vec![];

            $(
                v.push($crate::named_keys! {
                    @init_all
                    v $k $name : $t $(= $init)?
                });
            )*

            v
        }
    };
}

/// Helper macro that derives `ToBytes` and `FromBytes` for multiple structs.
#[macro_export]
macro_rules! serializable_structs {
    (
        $( $(#[$($meta:meta)+])* $sv:vis struct $name:ident {
            $( $fv:vis $field:ident : $t:ty ),*
            $(,)?
        })+
    ) => {
        $(
            $(#[$($meta)+])*
            $sv struct $name {
                $( $fv $field : $t ),*
            }

            impl $name {
                pub fn new(
                    $( $field : $t ),*
                ) -> Self {
                    Self {
                        $( $field ),*
                    }
                }
            }

            impl casper_types::bytesrepr::ToBytes for $name {
                fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
                    let mut result = alloc::vec::Vec::new();
                    $( result.append(&mut self.$field.to_bytes()?); )*
                    Ok(result)
                }

                fn serialized_length(&self) -> usize {
                    0 $( + self.$field.serialized_length() )*
                }
            }

            impl casper_types::bytesrepr::FromBytes for $name {
                fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
                    let mut remainder = bytes;
                    $( let ($field, new_remainder) = casper_types::bytesrepr::FromBytes::from_bytes(remainder)?;
                    remainder = new_remainder; )*
                    Ok((
                        Self {
                            $( $field ),*
                        },
                        remainder
                    ))
                }
            }

            impl casper_types::CLTyped for $name {
                fn cl_type() -> casper_types::CLType {
                    casper_types::CLType::Any
                }
            }
        )+
    };
}

/// Declare an entrypoint signature for a contract.
#[macro_export]
macro_rules! entrypoint {
    (
        @access public $($attrib:ident)*
    ) => {
        casper_types::EntryPointAccess::Public
    };

    (
        @access $_:ident $($attrib:ident)*
    ) => {
        $crate::entrypoint! { @access $($attrib)* }
    };

    (
        @access
    ) => {
        core::compile_error!("Missing access specifier for entrypoint")
    };

    (
        @context contract $($attrib:ident)*
    ) => {
        casper_types::EntryPointType::Contract
    };

    (
        @context session $($attrib:ident)*
    ) => {
        casper_types::EntryPointType::Session
    };

    (
        @context $_:ident $($attrib:ident)*
    ) => {
        $crate::entrypoint! { @context $($attrib)* }
    };

    (
        @context
    ) => {
        core::compile_error!("Missing context specifier for entrypoint")
    };

    (
        $(#[$meta:meta])*
        $([$($marker:ident)*])? fn $name:ident
        ( $( $arg:ident : $t:ty ),* $(,)?) -> $ret:ty = $callback:path
    ) => {
        $(#[$meta])*
        pub fn $name() {
            use casper_contract::unwrap_or_revert::UnwrapOrRevert;

            $(
                let $arg: $t = casper_contract::contract_api::runtime::get_named_arg(stringify!($arg));
            )*

            let result = $callback($($arg),*);

            casper_contract::contract_api::runtime::ret(casper_types::CLValue::from_t(result).unwrap_or_revert());
        }

        paste::paste! {
            pub fn [< $name _ep >] () -> casper_types::EntryPoint {
                use casper_types::CLTyped;

                casper_types::EntryPoint::new(
                    stringify!($name),
                    alloc::vec![
                        $(
                            casper_types::Parameter::new(stringify!($arg), <$t>::cl_type()),
                        )*
                    ],
                    <$ret>::cl_type(),
                    $crate::entrypoint! { @access $($($marker)*)? },
                    $crate::entrypoint! { @context $($($marker)*)? },
                )
            }
        }
    };
}

/// Declare multiple entrypoints for a contract. Creates a function `all_entrypoints` that returns a
/// vector of all entrypoints.
#[macro_export]
macro_rules! entrypoints {
    (
        $(
            $(#[$meta:meta])*
            $([$($marker:ident)*])? fn $name:ident
            ( $( $arg:ident : $t:ty ),* $(,)?) -> $ret:ty = $callback:path
        );* $(;)?
    ) => {
        $(
            $crate::entrypoint! {
                $(#[$meta])*
                $([$($marker)*])? fn $name
                ( $( $arg : $t ),*) -> $ret = $callback
            }
        )*

        pub fn all_entrypoints() -> alloc::vec::Vec<casper_types::EntryPoint> {
            paste::paste! {
                alloc::vec![
                    $(
                        [< $name _ep >] (),
                    )*
                ]
            }
        }
    };
}

/// Forward entrypoints to another module.
#[macro_export]
macro_rules! forward_entrypoints {
    ($base:ident : [$($ep:ident),+ $(,)?]) => {
        $(
            #[no_mangle]
            extern "C" fn $ep() {
                $base::$ep();
            }
        )+
    };
}

/// Read a named argument and create a local binding.
#[macro_export]
macro_rules! read_arg {
    ($name:ident: $t:ty) => {
        let $name: $t = casper_contract::contract_api::runtime::get_named_arg(stringify!($name));
    };
}

/// Same as `read_arg`, but for `TokenIdentifier`. This is mostly for consistency.
#[macro_export]
macro_rules! read_token_id {
    ($name:ident) => {
        let $name: $crate::TokenIdentifier = $crate::TokenIdentifier::load_from_runtime_args();
    };
}

pub fn b64(input: &[u8]) -> String {
    base64::prelude::BASE64_STANDARD_NO_PAD.encode(input)
}

/// Utilities for working with the contract call stack.
pub mod call_stack {
    use casper_contract::{
        contract_api::runtime::{self, revert},
        unwrap_or_revert::UnwrapOrRevert,
    };
    use casper_types::{
        system::CallStackElement, ApiError, ContractHash, ContractPackageHash, Key,
    };
    use once_cell::unsync::Lazy;

    st_non_sync_static! {
        Lazy<alloc::vec::Vec<CallStackElement>> = Lazy::new(|| {
            runtime::get_call_stack()
        })
    }

    /// Equivalent of [`runtime::get_call_stack`], but cached across invocations.
    pub fn read() -> &'static [CallStackElement] {
        &*get()
    }

    /// Get a call stack element at `depth`
    pub fn at_depth(depth: usize) -> Option<&'static CallStackElement> {
        let call_stack = read();
        if depth >= call_stack.len() {
            None
        } else {
            call_stack.get(call_stack.len() - depth - 1)
        }
    }

    /// Return the context of the immediate caller of the current context.
    ///
    /// Reverts with [`ApiError::User(u16::MAX)`] if there is no immediate caller.
    pub fn caller() -> &'static CallStackElement {
        at_depth(1).unwrap_or_revert_with(ApiError::User(u16::MAX))
    }

    /// Return the current context.
    pub fn current() -> &'static CallStackElement {
        // this is infallible
        at_depth(0).unwrap_or_revert()
    }

    fn current_contract_full() -> (&'static ContractPackageHash, &'static ContractHash) {
        match current() {
            CallStackElement::Session { .. } => revert(ApiError::User(u16::MAX)),
            CallStackElement::StoredSession {
                contract_package_hash,
                contract_hash,
                ..
            } => (contract_package_hash, contract_hash),
            CallStackElement::StoredContract {
                contract_package_hash,
                contract_hash,
            } => (contract_package_hash, contract_hash),
        }
    }

    /// Return the current contract's package hash.
    ///
    /// Reverts with [`ApiError::User(u16::MAX)`] if the current context doesn't reference a contract.
    pub fn current_package() -> ContractPackageHash {
        *current_contract_full().0
    }

    /// Return the current contract's package hash.
    ///
    /// Reverts with [`ApiError::User(u16::MAX)`] if the current context doesn't reference a contract.
    pub fn current_contract() -> ContractHash {
        *current_contract_full().1
    }

    /// Extension trait for a call stack element.
    pub trait CallStackElementEx {
        /// Derive a [`Key`] identifying the context logical 'owner' or 'invoker'.
        ///
        /// For contexts that reference an account (Session and StoredSession), the key will be the
        /// account (as [`Key::Account`]). For contexts that reference a contract (StoredContract), the
        /// key will be the contract's package hash (as [`Key::Hash`])
        fn key(&self) -> Key;
    }

    impl CallStackElementEx for CallStackElement {
        fn key(&self) -> Key {
            match self {
                CallStackElement::Session { account_hash } => Key::Account(*account_hash),
                CallStackElement::StoredSession { account_hash, .. } => Key::Account(*account_hash),
                CallStackElement::StoredContract {
                    contract_package_hash,
                    ..
                } => Key::Hash(contract_package_hash.value()),
            }
        }
    }
}

// SAFETY: Copied almost verbatim from `casper_contract`, slightly modified to allow error handling (seriously we have to do this?)
// and some other things
pub mod contract_api {
    use core::mem::MaybeUninit;

    use alloc::vec::Vec;
    use casper_contract::{
        contract_api::{self, runtime::revert},
        ext_ffi,
        unwrap_or_revert::UnwrapOrRevert,
    };
    use casper_types::{
        api_error,
        bytesrepr::{self, FromBytes},
        ApiError, URef,
    };

    fn get_named_arg_size(name: &str) -> Option<usize> {
        let mut arg_size: usize = 0;
        let ret = unsafe {
            ext_ffi::casper_get_named_arg_size(
                name.as_bytes().as_ptr(),
                name.len(),
                &mut arg_size as *mut usize,
            )
        };
        match api_error::result_from(ret) {
            Ok(_) => Some(arg_size),
            Err(ApiError::MissingArgument) => None,
            Err(e) => revert(e),
        }
    }

    pub fn try_get_named_arg<T: FromBytes>(name: &str) -> Result<T, ApiError> {
        let arg_size = get_named_arg_size(name).ok_or(ApiError::MissingArgument)?;
        let arg_bytes = if arg_size > 0 {
            let res = {
                let data_non_null_ptr = contract_api::alloc_bytes(arg_size);
                let ret = unsafe {
                    ext_ffi::casper_get_named_arg(
                        name.as_bytes().as_ptr(),
                        name.len(),
                        data_non_null_ptr.as_ptr(),
                        arg_size,
                    )
                };
                let data =
                    unsafe { Vec::from_raw_parts(data_non_null_ptr.as_ptr(), arg_size, arg_size) };
                api_error::result_from(ret).map(|_| data)
            };
            // Assumed to be safe as `get_named_arg_size` checks the argument already
            res?
        } else {
            // Avoids allocation with 0 bytes and a call to get_named_arg
            Vec::new()
        };

        bytesrepr::deserialize(arg_bytes).map_err(|_| ApiError::InvalidArgument)
    }

    fn read_host_buffer_into(dest: &mut [u8]) -> Result<usize, ApiError> {
        let mut bytes_written = MaybeUninit::uninit();
        let ret = unsafe {
            ext_ffi::casper_read_host_buffer(
                dest.as_mut_ptr(),
                dest.len(),
                bytes_written.as_mut_ptr(),
            )
        };
        // NOTE: When rewriting below expression as `result_from(ret).map(|_| unsafe { ... })`, and the
        // caller ignores the return value, execution of the contract becomes unstable and ultimately
        // leads to `Unreachable` error.
        api_error::result_from(ret)?;
        Ok(unsafe { bytes_written.assume_init() })
    }

    fn read_host_buffer(size: usize) -> Result<Vec<u8>, ApiError> {
        let mut dest: Vec<u8> = if size == 0 {
            Vec::new()
        } else {
            let bytes_non_null_ptr = contract_api::alloc_bytes(size);
            unsafe { Vec::from_raw_parts(bytes_non_null_ptr.as_ptr(), size, size) }
        };
        read_host_buffer_into(&mut dest)?;
        Ok(dest)
    }

    pub fn new_dictionary_anon() -> URef {
        let value_size = {
            let mut value_size = MaybeUninit::uninit();
            let ret = unsafe { ext_ffi::casper_new_dictionary(value_size.as_mut_ptr()) };
            api_error::result_from(ret).unwrap_or_revert();
            unsafe { value_size.assume_init() }
        };
        let value_bytes = read_host_buffer(value_size).unwrap_or_revert();
        bytesrepr::deserialize(value_bytes).unwrap_or_revert()
    }
}
