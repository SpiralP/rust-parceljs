use includedir_codegen::Compression;
use std::{env, fs, path::PathBuf, process::Command};
use walkdir::WalkDir;

fn run(cmd: &str) -> bool {
  if cfg!(target_os = "windows") {
    check_command(Command::new("cmd").arg("/C").arg(cmd))
  } else {
    check_command(Command::new("sh").arg("-c").arg(cmd))
  }
}

fn run_envs(cmd: &str, envs: Vec<(&str, &str)>) -> bool {
  if cfg!(target_os = "windows") {
    check_command(Command::new("cmd").arg("/C").arg(cmd).envs(envs))
  } else {
    check_command(Command::new("sh").arg("-c").arg(cmd).envs(envs))
  }
}

fn check_command(command: &mut Command) -> bool {
  command.status().unwrap().success()

  // npm likes to warn a lot so disabled

  // if !command.status().unwrap().success() {
  //   return false;
  // }

  // let output = command.output().unwrap();

  // output.stderr.is_empty()
}

pub struct Builder {
  /// optionally change directory before doing anything
  pub current_dir: Option<PathBuf>,

  /// directory containing index.html
  pub in_dir: PathBuf,

  /// output files go into this directory
  /// TODO must always be "dist"
  pub out_dir: PathBuf,
}

impl Default for Builder {
  fn default() -> Self {
    Self {
      current_dir: None,
      in_dir: "web".into(),
      out_dir: "dist".into(),
    }
  }
}

impl Builder {
  pub fn build(self) {
    let last_current_dir = env::current_dir().unwrap();

    let in_dir = &self.in_dir;
    let out_dir = &self.out_dir;

    assert_eq!(format!("{}", out_dir.display()), "dist");

    if let Some(current_dir) = self.current_dir {
      env::set_current_dir(current_dir).unwrap();
    }

    if !env::var("OUT_DIR").unwrap().contains("rls") && fs::metadata("package.json").is_ok() {
      // if no node_modules, run npm install
      if fs::metadata("node_modules").is_err() {
        assert!(run("npm install"));
      }

      let _ = fs::remove_dir_all(&out_dir);

      if cfg!(debug_assertions) {
        assert!(run_envs(
          "npm run-script build",
          vec![("NODE_ENV", "development")]
        ));
      } else {
        assert!(run("npm run-script build"));
      }
    }

    assert!(
      fs::metadata(&out_dir)
        .map(|meta| meta.is_dir())
        .unwrap_or(false),
      "out directory wasn't created"
    );

    // let last_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // env::set_var("CARGO_MANIFEST_DIR", &workspace_dir);

    includedir_codegen::start("WEB_FILES")
      .dir(&out_dir, Compression::Gzip)
      .build("web_files.rs")
      .unwrap();

    for entry in WalkDir::new(&in_dir) {
      match entry {
        Ok(ref e) if !e.file_type().is_dir() => {
          println!("cargo:rerun-if-changed={}", e.path().display());
        }

        _ => {}
      }
    }

    // env::set_var("CARGO_MANIFEST_DIR", last_manifest_dir);
    env::set_current_dir(last_current_dir).unwrap();
  }
}

/// Build with default options.
pub fn build() {
  Builder::default().build()
}
