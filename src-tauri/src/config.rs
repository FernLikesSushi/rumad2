//! Compile-time defaults for the SSH connection to the RUMAD OpenVMS
//! system. The shared "estudiante" account only renders the public
//! `MENU PRINCIPAL` — anything account-specific (e.g. class selection)
//! requires a real student's own credentials, entered on the app's login
//! screen instead of relying on these defaults.
//!
//! Override at build time with e.g. `RUMAD_SSH_HOST=... cargo build`.

pub const DEFAULT_HOST: &str = match option_env!("RUMAD_SSH_HOST") {
    Some(host) => host,
    None => "rumad.uprm.edu",
};

pub const DEFAULT_USERNAME: &str = match option_env!("RUMAD_SSH_USER") {
    Some(username) => username,
    None => "estudiante",
};

pub const DEFAULT_PASSWORD: &str = match option_env!("RUMAD_SSH_PASSWORD") {
    Some(password) => password,
    None => "",
};

pub const SSH_PORT: u16 = 22;
