use std::env;

pub fn get_state_path() -> String {
    env::var("SDK_STATE_DIR").expect("SDK_STATE_DIR environment variable not set")
}

pub fn get_config_path() -> String {
    env::var("SDK_CONFIG_DIR").expect("SDK_CONFIG_DIR environment variable not set")
}

pub fn get_home_path() -> String {
    env::var("SDK_HOME_DIR").unwrap_or_else(|_| "/root".to_owned())
}

pub fn get_host_os() -> String {
    env::var("OPS_HOST_PLATFORM").unwrap_or_else(|_| "unknown".to_owned())
}

pub fn get_interface_type() -> String {
    env::var("SDK_INTERFACE_TYPE").unwrap_or_else(|_| "terminal".to_owned())
}
