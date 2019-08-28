use includedir_codegen::Compression;
use std::{env, fs, process::Command};

fn run(cmd: &str) -> bool {
  if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", cmd])
      .status()
      .unwrap()
      .success()
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(cmd)
      .status()
      .unwrap()
      .success()
  }
}

fn main() {
  let last_current_dir = env::current_dir().unwrap();

  // /unison/Projects/rust-parceljs/target/debug/build/parceljs-83b0eeadf2f2e1d5/out
  //                               /..    /..   /..   /..                       /..
  let mut workspace_dir = std::path::PathBuf::from(&env::var("OUT_DIR").unwrap());
  workspace_dir.pop();
  workspace_dir.pop();
  workspace_dir.pop();
  workspace_dir.pop();
  workspace_dir.pop();

  env::set_current_dir(&workspace_dir).unwrap();

  if !env::var("OUT_DIR").unwrap().contains("rls") {
    if fs::metadata("package.json").is_ok() {
      if fs::metadata("node_modules").is_err() {
        assert!(run("yarn install"));
      }

      let _ = fs::remove_dir_all("dist");
      assert!(run("yarn build"));
    }
  }

  let last_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  env::set_var("CARGO_MANIFEST_DIR", workspace_dir);

  includedir_codegen::start("WEB_FILES")
    .dir("dist", Compression::Gzip)
    .build("web_files.rs")
    .unwrap();

  env::set_var("CARGO_MANIFEST_DIR", last_manifest_dir);
  env::set_current_dir(last_current_dir).unwrap();
}
