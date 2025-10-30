use jane_eyre::eyre;
use osakit::{self, declare_script};

pub fn clone_guest(original_guest_name: &str, new_guest_name: &str) -> eyre::Result<()> {
    declare_script! {
        #[language(AppleScript)]
        #[source(r#"
            on clone_guest(original_guest_name, new_guest_name)
                tell application "UTM"
                    set vm to virtual machine named original_guest_name
                    duplicate vm with properties {configuration: {name: new_guest_name}}
                end tell
            end clone_guest
        "#)]
        Script {
            fn clone_guest(original_guest_name: &str, new_guest_name: &str);
        }
    }
    Script::new()?.clone_guest(original_guest_name, new_guest_name)?;
    Ok(())
}
