pub fn show_usage() {
    #[cfg(windows)]
    let command_name = "billing.exe";
    #[cfg(not(windows))]
    let command_name = "./billing";
    print!(
        "liuguang's billing server
build by {}
Git Commit: {:.7}

USAGE:
       {command_name} {:10} Run the billing server in the foreground
       {command_name} {:10} Detached mode: Run the billing server in the background
       {command_name} {:10} Stop the billing server",
        option_env!("COMPILER_VERSION").unwrap_or("-"),
        option_env!("GIT_COMMIT_VERSION").unwrap_or("-"),
        "up",
        "up -d",
        "stop",
        command_name = command_name
    );
}
