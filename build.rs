#[cfg(feature = "zip")]
use std::path::PathBuf;

#[cfg(feature = "zip")]
use cmake::Config;

#[cfg(not(feature = "zip"))]
fn main() {
  // nothing
}

#[cfg(feature = "zip")]
fn main() {
  let profile = std::env::var("PROFILE").unwrap();
  let c_profile = match profile.as_str() {
    "debug"   => "Debug",
    "release" => "Release",
    _         => profile.as_str(),
  };
  let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  println!("cargo:rerun-if-changed={}/src", project_root.to_str().unwrap());
  println!("cargo:rerun-if-changed={}/native", project_root.to_str().unwrap());
  let dst = Config::new("native").define("CMAKE_BUILD_TYPE", c_profile)
                                 .uses_cxx11()
                                 .very_verbose(true)
                                 .build_target("unzip_tool")
                                 .build();

  if cfg!(target_os = "windows") {
    println!("cargo:rustc-link-search={}/build/{}", dst.display(), c_profile);
    println!("cargo:rustc-link-search={}/build/zlib/{}", dst.display(), c_profile);
  } else {
    println!("cargo:rustc-link-search={}/build", dst.display());
  }

  if cfg!(target_os = "linux") {
    println!("cargo:rustc-link-lib=stdc++");
  } else if cfg!(target_os = "macos") {
    println!("cargo:rustc-link-lib=c++");
  }
  println!("cargo:rustc-link-lib=unzip_tool");
  println!("cargo:rustc-link-lib=minizip");
  if cfg!(target_os = "windows") {
    match profile.as_str() {
      "debug" => println!("cargo:rustc-link-lib=zlibstaticd"),
      _       => println!("cargo:rustc-link-lib=zlibstatic")
    };
  } else {
    println!("cargo:rustc-link-lib=z");
  }
}
