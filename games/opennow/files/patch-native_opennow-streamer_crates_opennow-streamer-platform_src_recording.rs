--- native/opennow-streamer/crates/opennow-streamer-platform/src/recording.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/recording.rs
@@ -174,7 +174,7 @@
 }
 
 pub(crate) fn publish_manual_recording(part: &Path, output: &Path) -> std::io::Result<()> {
-    #[cfg(any(target_os = "linux", target_os = "macos"))]
+    #[cfg(any(any(target_os = "linux", target_os = "freebsd"), target_os = "macos"))]
     {
         use std::ffi::CString;
         use std::os::unix::ffi::OsStrExt;
@@ -191,6 +191,17 @@
                 libc::RENAME_NOREPLACE,
             )
         };
+        // FreeBSD has no renameat2(RENAME_NOREPLACE): link(2) fails with EEXIST
+        // instead of replacing an existing recording, then the partial name goes.
+        #[cfg(target_os = "freebsd")]
+        let result = {
+            let linked = unsafe { libc::link(part.as_ptr(), output.as_ptr()) };
+            if linked == 0 {
+                unsafe { libc::unlink(part.as_ptr()) }
+            } else {
+                linked
+            }
+        };
         #[cfg(target_os = "macos")]
         let result = unsafe { libc::renamex_np(part.as_ptr(), output.as_ptr(), libc::RENAME_EXCL) };
         if result == 0 {
@@ -226,7 +237,7 @@
             Err(std::io::Error::last_os_error())
         }
     }
-    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
+    #[cfg(not(any(any(target_os = "linux", target_os = "freebsd"), target_os = "macos", target_os = "windows")))]
     {
         let _ = (part, output);
         Err(std::io::Error::new(
