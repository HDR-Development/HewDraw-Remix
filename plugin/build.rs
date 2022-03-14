static VERSION: &str = include_str!("hdr_version.txt");

fn main() {
  println!("cargo:rustc-env=CARGO_PKG_VERSION={}", VERSION);
  println!("cargo:rustc-env=CARGO_NET_GIT_FETCH_WITH_CLI=true"); // force cargo to use git to fetch repos
}