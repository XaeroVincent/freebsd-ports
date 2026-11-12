--- native/opennow-streamer/crates/opennow-streamer-ffi/build.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-ffi/build.rs
@@ -1,6 +1,6 @@
 fn main() {
     match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
-        Ok("linux") => {
+        Ok("linux") | Ok("freebsd") => {
             println!("cargo:rustc-link-arg=-Wl,-soname,libopennow_streamer_ffi.so");
         }
         Ok("macos") => {
