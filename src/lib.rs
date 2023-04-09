/*
MIT License

Copyright (c) 2021 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Library + CLI-Tool to measure the TTFB (time to first byte) of HTTP(S) requests.
//! Additionally, this crate measures the times of DNS lookup, TCP connect, and
//! TLS handshake. This crate currently only supports HTTP/1.1. It can cope with
//! TLS 1.2 and 1.3.LICENSE.
//!
//! See [`ttfb`] which is the main function of the public interface.
//!
//! ## Cross Platform
//! CLI + lib work on Linux, MacOS, and Windows.

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
// I can't do anything about this; fault of the dependencies
#![allow(clippy::multiple_crate_versions)]
// allow: required because of derive macro.. :(
#![allow(clippy::use_self)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub use error::{InvalidUrlError, ResolveDnsError, TtfbError};
pub use outcome::TtfbOutcome;

use std::io::{Read as IoRead, Write as IoWrite};

mod error;
mod outcome;

/// Common super trait for TCP-Stream or `TLS<TCP>`-Stream.
trait TcpWithMaybeTlsStream: IoWrite + IoRead {}

#[macro_use]
extern crate cfg_if;


cfg_if! {
    if #[cfg(all(any(target_arch = "wasm32", target_arch = "wasm64"),
      target_os = "unknown"))] {
        #[path = "util_wasm32.rs"] mod imp;
    } else {
      #[path = "util.rs"] mod imp;
    }
}


/// Takes a URL and connects to it via http/1.1. Measures time for
/// DNS lookup, TCP connection start, TLS handshake, and TTFB (Time to First Byte)
/// of HTML content.
///
/// ## Parameters
/// - `input`: Url. Can be one of
///   - `phip1611.de` (defaults to `http://`)
///   - `http://phip1611.de`
///   - `https://phip1611.de`
///   - `https://phip1611.de?foo=bar`
///   - `https://sub.domain.phip1611.de?foo=bar`
///   - `http://12.34.56.78/foobar`
///   - `https://1.1.1.1`
///   - `12.34.56.78/foobar` (defaults to `http://`)
///   - `12.34.56.78` (defaults to `http://`)
/// - `allow_insecure_certificates`: if illegal certificates (untrusted, expired) should be accepted
///                                  when https is used. Similar to `-k/--insecure` in `curl`.
///
/// ## Return value
/// [`TtfbOutcome`] or [`TtfbError`].
pub fn ttfb(input: String, allow_insecure_certificates: bool) -> Result<imp::TtfbOutcome, imp::TtfbError> {
    return imp::ttfb(input, allow_insecure_certificates);
}
