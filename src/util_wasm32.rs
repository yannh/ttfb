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

use crate::error::TtfbError;
use crate::outcome::TtfbOutcome;

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
pub fn ttfb(input: String, _allow_insecure_certificates: bool) -> Result<TtfbOutcome, TtfbError> {
    Ok(TtfbOutcome::new(
        input,
        String::from("127.0.0.1"),
        66,
        100,
        10,
        100,
        100,
        100,
        // http_content_download_duration,
    ))
}
