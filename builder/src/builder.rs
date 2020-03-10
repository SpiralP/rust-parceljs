use flate2::{write::GzEncoder, Compression};
use std::{
  env, fs,
  fs::File,
  io,
  io::{BufReader, BufWriter, Write},
  path::{Path, PathBuf},
  process::Command,
};
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

    for entry in WalkDir::new(&in_dir) {
      match entry {
        Ok(entry) if entry.file_type().is_file() => {
          let path = entry.path().to_path_buf();

          println!("cargo:rerun-if-changed={}", path.display());
        }
        _ => {}
      }
    }

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("parceljs.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut phf = phf_codegen::Map::new();
    phf.phf_path("::parceljs::phf");

    let paths: Vec<_> = WalkDir::new(&out_dir)
      .into_iter()
      .filter_map(|entry| match entry {
        Ok(entry) if entry.file_type().is_file() => {
          let path = entry.path().to_path_buf();
          let relative_path = path.strip_prefix(out_dir).unwrap();
          let relative_path = relative_path.to_str().unwrap().to_string();
          let relative_path = relative_path.replace("\\", "/");
          Some((relative_path, path))
        }
        _ => None,
      })
      .collect();

    // gzip files into OUT_DIR
    for (relative_path, path) in &paths {
      let gzip_path = Path::new(&env::var("OUT_DIR").unwrap())
        .join("parceljs")
        .join(relative_path);

      let gzip_path_dir = gzip_path.parent().unwrap();
      if !gzip_path_dir.exists() {
        fs::create_dir_all(gzip_path_dir).unwrap();
      }

      let mut encoder = GzEncoder::new(File::create(&gzip_path).unwrap(), Compression::best());

      let mut in_file = BufReader::new(File::open(path).unwrap());
      io::copy(&mut in_file, &mut encoder).unwrap();

      let relative_path = relative_path.as_str();
      phf.entry(
        relative_path,
        &format!("include_bytes!({:?}) as &'static [u8]", gzip_path),
      );
    }

    writeln!(file, "#[allow(clippy::unreadable_literal)]").unwrap();

    writeln!(
      file,
      "static PARCELJS: ::parceljs::ParcelJs = ::parceljs::ParcelJs::new({});",
      phf.build()
    )
    .unwrap();

    env::set_current_dir(last_current_dir).unwrap();
  }
}

/// Build with default options.
pub fn build() {
  Builder::default().build()
}
