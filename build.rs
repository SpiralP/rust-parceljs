use includedir_codegen::Compression;
use std::{
  env, fs,
  path::{Path, PathBuf},
  process::Command,
};
use walkdir::WalkDir;

fn run(cmd: &str) -> bool {
  if cfg!(target_os = "windows") {
    check_command(Command::new("cmd").args(&["/C", cmd]))
  } else {
    check_command(Command::new("sh").arg("-c").arg(cmd))
  }
}

fn check_command(command: &mut Command) -> bool {
  if !command.status().unwrap().success() {
    return false;
  }

  let output = command.output().unwrap();

  output.stderr.is_empty()
}

fn pop<T>(path: &mut PathBuf, maybe: T)
where
  T: Into<Option<&'static str>>,
  T: std::fmt::Debug,
{
  if let Some(name) = maybe.into() {
    assert_eq!(path.file_name().unwrap(), name);
  }
  path.pop();
}

fn main() {
  let last_current_dir = env::current_dir().unwrap();

  // /unison/Projects/rust-parceljs/target/rls/debug/build/parceljs-afcec2ed84625bb4/out
  // /unison/Projects/rust-parceljs/target/debug/build/parceljs-83b0eeadf2f2e1d5/out
  //                               /..    /..   /..   /..                       /..
  let mut workspace_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
  pop(&mut workspace_dir, "out");
  pop(&mut workspace_dir, None); // pop parceljs-123123
  pop(&mut workspace_dir, "build");

  let file_name = workspace_dir.file_name().unwrap();
  assert!(file_name == "debug" || file_name == "release");
  pop(&mut workspace_dir, None); // pop "debug" or "release"

  if workspace_dir.file_name().unwrap() == "rls" {
    // fix for rls being target/rls/debug instead of target/debug
    pop(&mut workspace_dir, "rls");
  }

  pop(&mut workspace_dir, "target");

  env::set_current_dir(&workspace_dir).unwrap();

  if !env::var("OUT_DIR").unwrap().contains("rls") {
    if fs::metadata("package.json").is_ok() {
      // if no node_modules, run yarn install
      if fs::metadata("node_modules").is_err() {
        assert!(run("yarn install"));
      }

      let _ = fs::remove_dir_all("dist");
      assert!(run("yarn build"));
    }
  }

  assert!(
    fs::metadata("dist")
      .map(|meta| meta.is_dir())
      .unwrap_or(false),
    "dist not a directory"
  );

  let last_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  env::set_var("CARGO_MANIFEST_DIR", &workspace_dir);

  includedir_codegen::start("WEB_FILES")
    .dir("dist", Compression::Gzip)
    .build("web_files.rs")
    .unwrap();

  for entry in WalkDir::new("web") {
    match entry {
      Ok(ref e) if !e.file_type().is_dir() => {
        let full_path = Path::new(workspace_dir.as_path()).join(e.path());
        println!("cargo:rerun-if-changed={}", full_path.display());
      }

      _ => {}
    }
  }

  env::set_var("CARGO_MANIFEST_DIR", last_manifest_dir);
  env::set_current_dir(last_current_dir).unwrap();
}
