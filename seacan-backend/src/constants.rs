pub const LOG_DIRECTORY: &str = "/var/log/seacan";

pub const CONFIG_DIRECTORY: &str = "/etc/seacan";
pub const CONFIG_FILENAME: &str = "/etc/seacan/seacan.yml";
pub const CONFIG_DEFAULT_SETTINGS: &str = r###"
## how many days of log to keep
log_days_retention: 30

## number of worker threads
## use 0 to use the available system cores
number_of_workers: 0
"###;
