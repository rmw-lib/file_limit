#[cfg(not(target_os = "windows"))]
pub fn get() -> Result<usize, std::io::Error> {
  Ok(rlimit::increase_nofile_limit(usize::MAX as _)? as _)
}

#[cfg(target_os = "windows")]
pub fn get() -> Result<usize, std::io::Error> {
  rlimit::setmaxstdio(2048)?;
  Ok(rlimit::getmaxstdio() as usize)
}
