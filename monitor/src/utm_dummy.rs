use jane_eyre::eyre;

pub fn clone_guest(original_guest_name: &str, new_guest_name: &str) -> eyre::Result<()> {
    unimplemented!(r#"Requires `#[cfg(target_os = "macos")]`"#)
}
