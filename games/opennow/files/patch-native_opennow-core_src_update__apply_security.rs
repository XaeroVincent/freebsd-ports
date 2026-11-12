--- native/opennow-core/src/update_apply/security.rs.orig	2026-11-08 13:59:09.000000000 -0800
+++ native/opennow-core/src/update_apply/security.rs	2026-11-08 19:26:37.238122000 -0800
@@ -123,6 +123,11 @@ fn preserve_unix_acl(source: &Path, destination: &Path
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
