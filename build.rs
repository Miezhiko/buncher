use std::path::PathBuf;
use cmake::Config;

fn main() {
  let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  println!("cargo:rerun-if-changed={}/src", project_root.to_str().unwrap());
  println!("cargo:rerun-if-changed={}/native", project_root.to_str().unwrap());
  let dst = Config::new("native").define("CMAKE_BUILD_TYPE", "Release")
                                 .uses_cxx11()
                                 .very_verbose(true)
                                 .build_target("unzip_tool")
                                 .build();

  println!("cargo:rustc-link-search={}/build", dst.display());

  #[cfg(target_os = "windows")]
  println!("cargo:rustc-link-search={}/build/zlib", dst.display());

  if cfg!(target_os = "linux") {
    println!("cargo:rustc-link-lib=stdc++");
  } else if cfg!(target_os = "macos") {
    println!("cargo:rustc-link-lib=c++");
  }
  println!("cargo:rustc-link-lib=unzip_tool");
  println!("cargo:rustc-link-lib=minizip");
  println!("cargo:rustc-link-lib=z");
}
