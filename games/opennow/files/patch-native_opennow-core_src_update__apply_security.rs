--- native/opennow-core/src/update_apply/security.rs.orig
+++ native/opennow-core/src/update_apply/security.rs
@@ -123,6 +123,11 @@
     Ok(())
 }
 
+#[cfg(all(unix, not(target_os = "linux"), not(target_os = "macos")))]
+fn preserve_unix_acl(_source: &Path, _destination: &Path) -> Result<(), String> {
+    Ok(())
+}
+
 #[cfg(windows)]
 mod windows {
     use super::*;
