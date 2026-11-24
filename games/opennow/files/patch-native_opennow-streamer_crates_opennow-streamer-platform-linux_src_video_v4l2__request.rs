--- native/opennow-streamer/crates/opennow-streamer-platform-linux/src/video/v4l2_request.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform-linux/src/video/v4l2_request.rs
@@ -246,7 +246,10 @@
         .file
         .metadata()
         .map_err(|error| format!("fstat failed: {error}"))?;
-    let device = (libc::major(metadata.rdev()), libc::minor(metadata.rdev()));
+    let device = (
+        libc::major(metadata.rdev()) as u32,
+        libc::minor(metadata.rdev()) as u32,
+    );
     let mut failures = Vec::new();
     for path in media_paths {
         let result = (|| {
