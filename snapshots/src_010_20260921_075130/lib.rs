// lib.rs
// Date: Thu Sep 21 2026
// Educational and Practice Rust Programming Language Code

// rustc 1.100.0-nightly (bba531001 2026-09-20)
// binary: rustc
// commit-hash: bba531001d4de6d7f49693e0836a2668ca063282
// commit-date: 2026-09-20
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

use std::fmt::Display;

pub struct User {
    user_id: u64,
    username: String,
    password: String,
}

impl User {
    pub fn new(user_id: u64, username: impl Into<String>, password: impl Into<String>) -> User {
        User {
            user_id,
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn set_user_id(&mut self, user_id: u64) {
        self.user_id = user_id;
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        self.username = username.into();
    }

    pub fn set_password(&mut self, password: impl Into<String>) {
        self.password = password.into();
    }

    pub fn get_user_id(&self) -> &u64 {
        &self.user_id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!(" -> Goodbye {}", self.username);
        self.user_id = 0;
        self.username = String::default();
        self.password = String::default();
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[user_id: {}, username: {}, password: {}]",
            self.user_id, self.username, self.password
        )
    }
}
