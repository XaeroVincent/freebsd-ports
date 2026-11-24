--- native/opennow-core/src/update_apply/mod.rs.orig
+++ native/opennow-core/src/update_apply/mod.rs
@@ -158,7 +158,13 @@
 }
 
 pub fn external_update_message() -> Option<&'static str> {
-    if cfg!(target_os = "linux")
+    if cfg!(target_os = "freebsd") {
+        // FreeBSD installs come from pkg(8) or the ports tree, and releases publish no
+        // FreeBSD package, so there is no verified in-place update path to offer.
+        Some(
+            "OpenNOW is managed by the FreeBSD package system. Update it with `pkg upgrade` or by rebuilding the port, then restart OpenNOW.",
+        )
+    } else if cfg!(target_os = "linux")
         && is_flatpak_installation(
             std::env::var_os("FLATPAK_ID").as_deref(),
             Path::new("/.flatpak-info"),
@@ -233,6 +239,11 @@
         let root = installation_target(InstallKind::WindowsMsi, &executable)?;
         windows_update_package_extension(managed::windows_managed(&root)?)
     }
+    #[cfg(target_os = "freebsd")]
+    {
+        // Unreachable while external_update_message() reports FreeBSD as externally managed.
+        Err("FreeBSD packages are updated through pkg(8), not in place".to_owned())
+    }
 }
 
 pub fn prepare_update(request: PrepareRequest) -> Result<PreparedUpdate, String> {
