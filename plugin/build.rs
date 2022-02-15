fn main() {
  if let Ok(hdr_version) = std::env::var("HDR_VERSION") {
      println!("cargo:rustc-env=CARGO_PKG_VERSION={}", hdr_version);
  }
  println!("cargo:rustc-env=CARGO_NET_GIT_FETCH_WITH_CLI=true"); // force cargo to use git to fetch repos
}