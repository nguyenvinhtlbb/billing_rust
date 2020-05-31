use crate::common::LoggerSender;
use crate::log_message;

///输出软件信息
pub(super) async fn show_soft_info(logger_sender: &mut LoggerSender) {
    log_message!(
        logger_sender,
        Info,
        "powered by liuguang @github https://github.com/liuguangw"
    );
    log_message!(
        logger_sender,
        Info,
        "build by {}",
        option_env!("COMPILER_VERSION").unwrap_or("-")
    );
    log_message!(
        logger_sender,
        Info,
        "Git Commit: {:.7}",
        option_env!("GIT_COMMIT_VERSION").unwrap_or("-")
    );
}
