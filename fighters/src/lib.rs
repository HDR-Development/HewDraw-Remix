pub fn install() {
    #[cfg(not(feature = "ignore-common"))]
    { common::install(); }
    #[cfg(feature = "include-peach")]
    { peach::install(); }
}