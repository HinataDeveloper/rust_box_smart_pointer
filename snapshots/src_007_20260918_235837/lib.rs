// lib.rs
// Date: Thu Sep 18 2026

// rustc 1.100.0-nightly (330d31712 2026-09-17)
// binary: rustc
// commit-hash: 330d317121e16b5db8e5adc63595910528ff2ee7
// commit-date: 2026-09-17
// host: x86_64-unknown-linux-gnu
// release: 1.100.0-nightly
// LLVM version: 23.1.1

// cargo 1.100.0-nightly (495c385d0 2026-09-16)
// release: 1.100.0-nightly
// commit-hash: 495c385d0875c4ba51eb72ea0448a2d4c018b8d4
// commit-date: 2026-09-16
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.5-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

pub struct TripleBox<T>((T, T, T));

impl<T> TripleBox<T> {
    pub fn new(x: T, y: T, z: T) -> TripleBox<T> {
        TripleBox((x, y, z))
    }

    pub fn index_zero_elem(&self) -> &T {
        &self.0.0
    }

    pub fn index_first_elem(&self) -> &T {
        &self.0.1
    }

    pub fn index_second_elem(&self) -> &T {
        &self.0.2
    }

    pub fn head(&self) -> &(T, T, T) {
        &self.0
    }
}
