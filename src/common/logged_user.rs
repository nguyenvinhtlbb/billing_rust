use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

///已登录的用户信息
#[derive(Debug)]
pub struct LoggedUser {
    /// 角色是否在线
    role_online: bool,
    /// 最后活动时间
    last_active_time: SystemTime,
}

/// 所有已登录的用户集合
pub type LoggedUserCollection = Arc<RwLock<HashMap<String, LoggedUser>>>;
type LoggedUserReadGuard<'a> = RwLockReadGuard<'a, HashMap<String, LoggedUser>>;
type LoggedUserWriteGuard<'a> = RwLockWriteGuard<'a, HashMap<String, LoggedUser>>;

impl LoggedUser {
    /// 判断角色是否在线
    pub fn is_role_online(logged_users_guard: LoggedUserReadGuard, username: &str) -> bool {
        if let Some(logged_user) = logged_users_guard.get(username) {
            let time_now = SystemTime::now();
            let time_duration = time_now
                .duration_since(logged_user.last_active_time)
                .unwrap();
            //用户最后活跃时间在30分钟内
            if time_duration.as_secs() <= 30 * 60 {
                return logged_user.role_online;
            }
        }
        false
    }

    /// 设置用户信息
    pub fn set_logged_user(
        mut logged_users_guard: LoggedUserWriteGuard,
        username: &str,
        role_online: bool,
    ) {
        let mut logged_user = match logged_users_guard.remove(username) {
            Some(value) => value,
            None => Self::default(),
        };
        logged_user.role_online = role_online;
        logged_user.last_active_time = SystemTime::now();
        logged_users_guard.insert(username.to_string(), logged_user);
    }

    /// 移出用户
    pub fn remove_user(mut logged_users_guard: LoggedUserWriteGuard, username: &str) {
        logged_users_guard.remove(username);
    }
}

impl Default for LoggedUser {
    fn default() -> Self {
        LoggedUser {
            role_online: false,
            last_active_time: SystemTime::now(),
        }
    }
}
