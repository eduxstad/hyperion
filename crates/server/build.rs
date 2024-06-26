use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel
    let meta = version_meta().expect("could not get rustc version");
    match meta.channel {
        Channel::Stable | Channel::Beta | Channel::Dev => {
            panic!("This crate is only meant to be used with nightly rustc")
        }
        Channel::Nightly => {} /* Channel::Nightly => {
                                *     let data = meta.short_version_string;
                                *     assert_eq!("rustc 1.79.0-nightly (244da22fa 2024-04-23)", data);
                                * } */
    }
}
