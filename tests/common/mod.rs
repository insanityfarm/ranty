#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, time::{SystemTime, UNIX_EPOCH}};

use rant::compiler::{CompilerError, CompilerMessage};
use rant::runtime::RuntimeResult;
use rant::{Rant, RantOptions, RantProgram, RantValue};

static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

pub fn test_rant() -> Rant {
  Rant::with_options(RantOptions {
    debug_mode: true,
    ..Default::default()
  })
}

pub fn compile(source: &str) -> Result<RantProgram, CompilerError> {
  test_rant().compile_quiet(source)
}

pub fn compile_with_reporter(source: &str) -> (Result<RantProgram, CompilerError>, Vec<CompilerMessage>) {
  let rant = test_rant();
  let mut reporter = vec![];
  let result = rant.compile(source, &mut reporter);
  (result, reporter)
}

pub fn run(source: &str) -> RuntimeResult<RantValue> {
  let mut rant = test_rant();
  let program = rant.compile_quiet(source).expect("failed to compile program");
  rant.run(&program)
}

pub fn run_str(source: &str) -> String {
  run(source)
    .expect("failed to run program")
    .to_string()
}

pub struct TempWorkspace {
  root: PathBuf,
}

impl TempWorkspace {
  pub fn new() -> Self {
    let mut root = env::temp_dir();
    let nonce = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("system clock error")
      .as_nanos();
    let seq = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    root.push(format!("rant-tests-{nonce}-{seq}"));
    fs::create_dir_all(&root).expect("failed to create temporary workspace");
    Self { root }
  }

  pub fn path(&self) -> &Path {
    &self.root
  }

  pub fn write(&self, rel_path: &str, contents: &str) -> PathBuf {
    let path = self.root.join(rel_path);
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent).expect("failed to create parent directory");
    }
    fs::write(&path, contents).expect("failed to write test file");
    path
  }
}

impl Drop for TempWorkspace {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.root);
  }
}

pub struct EnvVarGuard {
  key: String,
  value: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
  pub fn set(key: &str, value: impl AsRef<std::ffi::OsStr>) -> Self {
    let prev = env::var_os(key);
    env::set_var(key, value);
    Self {
      key: key.to_owned(),
      value: prev,
    }
  }
}

impl Drop for EnvVarGuard {
  fn drop(&mut self) {
    if let Some(value) = &self.value {
      env::set_var(&self.key, value);
    } else {
      env::remove_var(&self.key);
    }
  }
}
