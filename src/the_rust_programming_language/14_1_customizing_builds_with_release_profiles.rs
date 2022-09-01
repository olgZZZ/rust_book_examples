// Customizing Builds with Release Profiles
// https://doc.rust-lang.org/stable/book/ch14-01-release-profiles.html





/*
These profile names might be familiar from the output of your builds:

$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s

*/



/*
Filename: Cargo.toml

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

*/




/*
You can override a default setting by adding a different value for it in Cargo.toml. For example, if we want to use optimization level 1 in the development profile, we can add these two lines to our project’s Cargo.toml file:

Filename: Cargo.toml

[profile.dev]
opt-level = 1

*/





