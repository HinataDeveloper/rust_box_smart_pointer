// Date: Thu Sep 19 2026

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

use std::ops::{Deref, DerefMut};

pub struct TripleBox<T>((T, T, T));

impl<T> TripleBox<T> {
    pub fn new(x: T, y: T, z: T) -> TripleBox<T> {
        TripleBox((x, y, z))
    }

    pub fn head(&self) -> &(T, T, T) {
        &self.0
    }

    pub fn zero(&self) -> &T {
        &self.0.0
    }

    pub fn one(&self) -> &T {
        &self.0.1
    }

    pub fn two(&self) -> &T {
        &self.0.2
    }
}

impl<T> Deref for TripleBox<T> {
    type Target = (T, T, T);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TripleBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
