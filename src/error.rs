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
//! Module for [`TtfbError`].
use derive_more::Display;
/// Errors during DNS resolving.
#[allow(missing_docs)]
#[derive(Debug, Display)]
pub enum ResolveDnsError {
    /// Can't find DNS entry for the given host.
    #[display(fmt = "Can't find DNS entry for the given host.")]
    NoResults,
    /// Couldn't resolve DNS for given host.
    #[display(fmt = "Couldn't resolve DNS for given host because!")]
    Other,
}

#[allow(missing_docs)]
#[derive(Debug, Display)]
pub enum InvalidUrlError {
    /// No input was provided. Provide a URL, such as <https://example.com> or <https://1.2.3.4:443>.
    #[display(
      fmt = "No input was provided. Provide a URL, such as https://example.com or https://1.2.3.4:443"
    )]
    MissingInput,
    /// The URL is illegal.
    #[display(fmt = "The URL is illegal because!")]
    WrongFormat(String),
    /// This tools only supports http and https.
    #[display(fmt = "This tools only supports http and https.")]
    WrongScheme,
    /// Other unknown error.
    #[display(fmt = "Other unknown error.")]
    Other,
}


/// Errors of the public interface of this crate.
#[derive(Debug)]
pub enum TtfbError {
    /// Invalid URL
    InvalidUrl(InvalidUrlError),
    /// Can't resolve DNS.
    CantResolveDns(ResolveDnsError),
}
