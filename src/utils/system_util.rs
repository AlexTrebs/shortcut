use std::time::SystemTime;

pub(crate) fn get_timestamp() -> i64 {
  let duration_since_epoch = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();
  duration_since_epoch.as_secs() as i64
}