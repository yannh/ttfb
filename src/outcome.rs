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
//! Module for [`TtfbOutcome`].
use serde::{Deserialize, Serialize};

/// The final result of this library. It contains all the measured timings.
#[derive(Debug,  Serialize, Deserialize)]
pub struct TtfbOutcome {
    /// Copy of the user input.
    user_input: String,
    /// The used IP address (resolved by DNS).
    ip_addr: String,
    /// The port.
    port: u16,
    /// If DNS was required, the relative duration of this operation.
    dns_duration_rel: u128,
    /// Relative duration of the TCP connection start.
    tcp_connect_duration_rel: u128,
    /// If https is used, the relative duration of the TLS handshake.
    tls_handshake_duration_rel: u128,
    /// The relative duration of the HTTP GET request sending.
    http_get_send_duration_rel: u128,
    /// The relative duration until the first byte from the HTTP response (the header) was
    /// received.
    http_ttfb_duration_rel: u128,
    // http_content_download_duration: Duration,
}

impl TtfbOutcome {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        user_input: String,
        ip_addr: String,
        port: u16,
        dns_duration_rel: u128,
        tcp_connect_duration_rel: u128,
        tls_handshake_duration_rel: u128,
        http_get_send_duration_rel: u128,
        http_ttfb_duration_rel: u128,
        // http_content_download_duration: Duration,
    ) -> Self {
        Self {
            user_input,
            ip_addr,
            port,
            dns_duration_rel,
            tcp_connect_duration_rel,
            tls_handshake_duration_rel,
            http_get_send_duration_rel,
            http_ttfb_duration_rel,
            // http_content_download_duration,
        }
    }

    /// Getter for [`Self::user_input`].
    pub fn user_input(&self) -> &str {
        &self.user_input
    }

    /// Getter for [`Self::ip_addr`] (relative time).
    pub fn ip_addr(&self) -> &str {
        &self.ip_addr
    }

    /// Getter for [`Self::port`] (relative time).
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Getter for [`Self::dns_duration_rel`] (relative time).
    pub const fn dns_duration_rel(&self) -> u128 {
        self.dns_duration_rel
    }

    /// Getter for [`Self::tcp_connect_duration_rel`] (relative time).
    pub const fn tcp_connect_duration_rel(&self) -> u128 {
        self.tcp_connect_duration_rel
    }

    /// Getter for [`Self::tls_handshake_duration_rel`] (relative time).
    pub const fn tls_handshake_duration_rel(&self) -> u128 {
        self.tls_handshake_duration_rel
    }

    /// Getter for [`Self::http_get_send_duration_rel`] (relative time).
    pub const fn http_get_send_duration_rel(&self) -> u128 {
        self.http_get_send_duration_rel
    }

    /// Getter for [`Self::http_ttfb_duration_rel`] (relative time).
    pub const fn http_ttfb_duration_rel(&self) -> u128 {
        self.http_ttfb_duration_rel
    }

    /// Getter for the absolute duration from the beginning to the TCP connect.
    /// Calculated by the relative TCP connect time + DNS relative times.
    pub fn tcp_connect_duration_abs(&self) -> u128 {
        self.dns_duration_rel
            + self.tcp_connect_duration_rel
    }
    /// Getter for the absolute duration from the beginning to the TLS handshake.
    /// Calculated by the relative TLS handshake time + all previous relative times.
    pub fn tls_handshake_duration_abs(&self) -> u128 {
        self.tls_handshake_duration_rel
    }

    /// Getter for the absolute duration from the beginning to the HTTP GET send.
    /// Calculated by the relative HTTP GET send time + all previous relative times.
    pub fn http_get_send_duration_abs(&self) -> u128 {
        self.tls_handshake_duration_abs()
            + self.http_get_send_duration_rel
    }

    /// Getter for the absolute duration from the beginning to the TTFB.
    /// Calculated by the relative TTFB time + all previous relative times.
    pub fn http_ttfb_duration_abs(&self) -> u128 {
        self.http_ttfb_duration_rel + self.http_get_send_duration_abs()
    }

    /*pub fn http_content_download_duration(&self) -> Duration {
        self.http_content_download_duration
    }*/
}

#[cfg(test)]
mod tests {
    use crate::outcome::TtfbOutcome;
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Duration;

    #[test]
    fn test_outcome() {
        let outcome = TtfbOutcome::new(
            "https://phip1611.de".to_string(),
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)).to_string(),
            443,
            Some(Duration::from_millis(1)),
            Duration::from_millis(2),
            Some(Duration::from_millis(3)),
            Duration::from_millis(4),
            Duration::from_millis(5),
        );
        assert_eq!(
            outcome.dns_duration_rel().unwrap().as_millis(),
            1,
            "DNS is the very first operation"
        );
        assert_eq!(
            outcome.tcp_connect_duration_abs().as_millis(),
            1 + 2,
            "DNS + TCP connect"
        );
        println!("{:#?}", outcome);
        assert_eq!(
            outcome.tls_handshake_duration_abs().unwrap().as_millis(),
            1 + 2 + 3,
            "DNS + TCP connect + TLS handshake"
        );
        assert_eq!(
            outcome.http_get_send_duration_abs().as_millis(),
            1 + 2 + 3 + 4,
            "DNS + TCP connect + TLS handshake + HTTP GET send"
        );
        assert_eq!(
            outcome.http_ttfb_duration_abs().as_millis(),
            1 + 2 + 3 + 4 + 5,
            "Total TTFB: DNS + TCP connect + TLS handshake + HTTP GET send + relative TTFB"
        );
    }
}
