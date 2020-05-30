use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

///已登录的用户信息
#[derive(Debug)]
pub struct AuthUser {
    /// 用户名
    username: String,
    /// 角色是否在线
    role_online: bool,
    /// 最后活动时间
    last_active_time: SystemTime,
}

/// 所有已登录的用户集合
pub type AuthUsersCollection = Arc<RwLock<HashMap<String, AuthUser>>>;
type AuthUserReadGuard<'a> = RwLockReadGuard<'a, HashMap<String, AuthUser>>;
type AuthUserWriteGuard<'a> = RwLockWriteGuard<'a, HashMap<String, AuthUser>>;

impl AuthUser {
    pub fn new(username: &str) -> Self {
        AuthUser {
            username: username.to_string(),
            role_online: false,
            last_active_time: SystemTime::now(),
        }
    }

    /// 判断角色是否在线
    pub fn is_role_online(auth_users_guard: AuthUserReadGuard, username: &str) -> bool {
        if let Some(auth_user) = auth_users_guard.get(username) {
            let time_now = SystemTime::now();
            let time_duration = time_now.duration_since(auth_user.last_active_time).unwrap();
            //用户最后活跃时间在30分钟内
            if time_duration.as_secs() <= 30 * 60 {
                return auth_user.role_online;
            }
        }
        false
    }

    /// 设置用户信息
    pub fn set_auth_user(
        mut auth_users_guard: AuthUserWriteGuard,
        username: &str,
        role_online: bool,
    ) {
        let mut auth_user = match auth_users_guard.remove(username) {
            Some(value) => value,
            None => Self::new(username),
        };
        auth_user.role_online = role_online;
        auth_user.last_active_time = SystemTime::now();
        auth_users_guard.insert(username.to_string(), auth_user);
    }

    /// 移出用户
    pub fn remove_user(mut auth_users_guard: AuthUserWriteGuard, username: &str) {
        auth_users_guard.remove(username);
    }
}
