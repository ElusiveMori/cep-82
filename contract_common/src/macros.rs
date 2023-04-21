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

        #[cfg(feature = "onchain")]
        static mut GLOBAL: $t = $init;

        #[cfg(feature = "onchain")]
        pub(super) fn get() -> &'static $t {
            assert!(cfg!(target_arch = "wasm32"));

            // SAFETY: This is safe because we are in a Casper wasm32 environment, which
            // is single-threaded. Since only one thread can access this static at a time,
            // we can safely return a reference to it.
            unsafe { &GLOBAL }
        }

        #[cfg(not(feature = "onchain"))]
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
            use once_cell::unsync::OnceCell;

            $crate::st_non_sync_static!(OnceCell<URef> = OnceCell::new());

            pub fn uref() -> URef {
                *(get().get_or_init(|| load_uref()))
            }

            pub fn set_cache(uref: URef) {
                $crate::r_unwrap!(get().set(uref), $crate::error::CommonError::InvalidCacheSet);
            }
        }

        pub use cache::uref;

        /// Loads the URef from storage. Prefer to use `uref()` instead, which uses a cached value.
        pub fn load_uref() -> URef {
            let key = $crate::o_unwrap!(runtime::get_key(NAME), ApiError::MissingKey);
            $crate::o_unwrap!(key.into_uref(), ApiError::UnexpectedKeyVariant)
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
            use casper_types::{ApiError, URef};

            $crate::named_key! { @uref $name, $t }

            pub fn try_read(key: &str) -> Option<$t> {
                $crate::r_unwrap!(storage::dictionary_get(uref(), key), ApiError::Deserialize)
            }

            pub fn read(key: &str) -> $t {
                $crate::o_unwrap!(try_read(key), ApiError::MissingKey)
            }

            pub fn write(key: &str, value: $t) {
                storage::dictionary_put(uref(), key, value);
            }

            pub fn remove(key: &str) {
                storage::dictionary_put(uref(), key, ());
            }

            pub fn init() -> URef {
                let uref = $crate::contract_api::new_dictionary_anon();
                cache::set_cache(uref);
                uref
            }
        }
    };

    (val $name:ident : $t:ty = $init:expr) => {
        pub mod $name {
            #[allow(unused)]
            use super::*;
            use casper_contract::contract_api::{runtime, storage};
            use casper_types::{ApiError, URef};

            $crate::named_key! { @uref $name, $t }

            pub fn try_read() -> Option<$t> {
                $crate::r_unwrap!(storage::read(uref()), ApiError::Deserialize)
            }

            pub fn read() -> $t {
                $crate::o_unwrap!(try_read(), ApiError::MissingKey)
            }

            pub fn write(value: $t) {
                storage::write(uref(), value);
            }

            pub fn init(v: $t) -> URef {
                let uref = casper_contract::contract_api::storage::new_uref(v);
                cache::set_cache(uref);
                uref
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
        ($name::NAME.to_string(), casper_types::Key::URef( $name::init($init) ))
    };

    (
        @init_all
        $v:ident
        dict $name:ident : $t:ty
    ) => {
        ($name::NAME.to_string(), casper_types::Key::URef( $name::init() ))
    };

    (
        $init_all:ident ( $( $arg_i:ident : $arg_t:ty ),* $(,)? ):
        $( $k:ident $name:ident : $t:ty $(= $init:expr)? );* $(;)?
    ) => {
        $(
            $crate::named_key! { $k $name : $t $(= $init)? }
        )*

        pub fn $init_all(
            $( $arg_i : $arg_t ),*
        ) -> alloc::vec::Vec<(alloc::string::String, casper_types::Key)> {
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
        [install] fn $name:ident
        ( $( $arg:ident : $t:ty ),* $(,)?) -> $ret:ty = $callback:path
    ) => {
        $(#[$meta])*
        pub fn $name() {
            $(
                let $arg: $t = if let Some(t) = $crate::FromNamedArg::try_get(stringify!($arg)) {
                    t
                } else {
                    $crate::debug_log!("Missing argument: {}", stringify!($arg));
                    casper_contract::contract_api::runtime::revert(casper_types::ApiError::MissingArgument);
                };
            )*

            let result = $callback($($arg),*);

            casper_contract::contract_api::runtime::ret(
                $crate::r_unwrap!(casper_types::CLValue::from_t(result))
            );
        }
    };

    (
        $(#[$meta:meta])*
        [$($marker:ident)*] fn $name:ident
        ( $( $arg:ident : $t:ty ),* $(,)?) -> $ret:ty = $callback:path
    ) => {
        $(#[$meta])*
        pub fn $name() {
            $crate::qlog!(">> {} entrypoint: {} - start", crate::NAME, stringify!($name));

            $(
                let $arg: $t = if let Some(t) = $crate::FromNamedArg::try_get(stringify!($arg)) {
                    t
                } else {
                    $crate::qlog!("missing argument: {}", stringify!($arg));
                    casper_contract::contract_api::runtime::revert(casper_types::ApiError::MissingArgument);
                };
            )*

            $crate::qlog!("arguments: {:?}", ($((stringify!($arg), &$arg)),*));

            let result: $ret = $callback($($arg),*);

            $crate::qlog!("<< {} entrypoint: {} - end", crate::NAME, stringify!($name));

            casper_contract::contract_api::runtime::ret(
                $crate::r_unwrap!(casper_types::CLValue::from_t(result))
            );
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
                    $crate::entrypoint! { @access $($marker)* },
                    $crate::entrypoint! { @context $($marker)* },
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
            #[cfg(feature = "onchain")]
            extern "C" fn $ep() {
                $base::$ep();
            }
        )+
    };
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "test-support")]
        {
            casper_contract::contract_api::runtime::print(
                &(
                    alloc::format!("[{}:{}] ", file!(), line!())
                    + &alloc::format!($($arg)*)
                )
            );
        }
    };
}

#[macro_export]
macro_rules! qlog {
    ($($arg:tt)*) => {
        #[cfg(feature = "test-support")]
        {
            casper_contract::contract_api::runtime::print(
                &alloc::format!($($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($a:expr, $b:expr, $err:expr) => {
        if $a != $b {
            $crate::debug_log!("ensure_eq failed: {} != {}", stringify!($a), stringify!($b));

            revert($err);
        }
    };
}

#[macro_export]
macro_rules! ensure_neq {
    ($a:expr, $b:expr, $err:expr) => {
        if $a == $b {
            $crate::debug_log!(
                "ensure_neq failed: {} == {}",
                stringify!($a),
                stringify!($b)
            );

            revert($err);
        }
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            $crate::debug_log!("ensure failed: {}", stringify!($cond));

            revert($err);
        }
    };
}

#[macro_export]
macro_rules! function_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }};
}

#[macro_export]
macro_rules! r_unwrap {
    (
        $e:expr $(,)?
    ) => {{
        $crate::r_unwrap!($e, casper_types::ApiError::None)
    }};

    (
        $e:expr,
        $c:expr $(,)?
    ) => {{
        match $e {
            Ok(value) => value,
            Err(error) => {
                $crate::qlog!(
                    "[{}] r_unwrap!({}) failed: {}",
                    $crate::function_path!(),
                    stringify!($e),
                    error
                );
                casper_contract::contract_api::runtime::revert($c);
            }
        }
    }};
}

#[macro_export]
macro_rules! o_unwrap {
    (
        $e:expr $(,)?
    ) => {{
        $crate::o_unwrap!($e, casper_types::ApiError::None)
    }};

    (
        $e:expr,
        $c:expr $(,)?
    ) => {{
        match $e {
            Some(value) => value,
            None => {
                $crate::qlog!(
                    "[{}] o_unwrap!({}) failed",
                    $crate::function_path!(),
                    stringify!($e)
                );
                casper_contract::contract_api::runtime::revert($c);
            }
        }
    }};
}

#[macro_export]
macro_rules! trace_block {
    ($tt:tt) => {{
        $crate::qlog!("{} - {}", $crate::function_path!(), "start");
        let result = $tt;
        $crate::qlog!("{} - {}", $crate::function_path!(), "end");

        result
    }};
}
