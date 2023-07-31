#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.79.0.

use crate::bridge::api::*;
use core::panic::UnwindSafe;
use crate::bridge::bridge_engine::rust2dart::IntoIntoDart;
use crate::bridge::bridge_engine::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_prepare_rust_signal_stream_impl(port_: MessagePort) {
    BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "prepare_rust_signal_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| {
                Ok(prepare_rust_signal_stream(
                    task_callback.stream_sink::<_, RustSignal>(),
                ))
            }
        },
    )
}
fn wire_prepare_rust_response_stream_impl(port_: MessagePort) {
    BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "prepare_rust_response_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| {
                Ok(prepare_rust_response_stream(
                    task_callback.stream_sink::<_, RustResponseUnique>(),
                ))
            }
        },
    )
}
fn wire_prepare_channels_impl(port_: MessagePort) {
    BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "prepare_channels",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(prepare_channels()),
    )
}
fn wire_check_rust_streams_impl(port_: MessagePort) {
    BRIDGE_HANDLER.wrap::<_, _, _, bool>(
        WrapInfo {
            debug_name: "check_rust_streams",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(check_rust_streams()),
    )
}
fn wire_start_rust_logic_impl(port_: MessagePort) {
    BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "start_rust_logic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(start_rust_logic()),
    )
}
fn wire_request_to_rust_impl(
    port_: MessagePort,
    request_unique: impl Wire2Api<RustRequestUnique> + UnwindSafe,
) {
    BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "request_to_rust",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_request_unique = request_unique.wire2api();
            move |task_callback| Ok(request_to_rust(api_request_unique))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<RustOperation> for i32 {
    fn wire2api(self) -> RustOperation {
        match self {
            0 => RustOperation::Create,
            1 => RustOperation::Read,
            2 => RustOperation::Update,
            3 => RustOperation::Delete,
            _ => unreachable!("Invalid variant for RustOperation: {}", self),
        }
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for RustResponse {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.successful.into_into_dart().into_dart(),
            self.bytes.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustResponse {}
impl rust2dart::IntoIntoDart<RustResponse> for RustResponse {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for RustResponseUnique {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_into_dart().into_dart(),
            self.response.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustResponseUnique {}
impl rust2dart::IntoIntoDart<RustResponseUnique> for RustResponseUnique {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for RustSignal {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.address.into_into_dart().into_dart(),
            self.bytes.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustSignal {}
impl rust2dart::IntoIntoDart<RustSignal> for RustSignal {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
