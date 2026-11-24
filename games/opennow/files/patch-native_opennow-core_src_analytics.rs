--- native/opennow-core/src/analytics.rs.orig
+++ native/opennow-core/src/analytics.rs
@@ -640,6 +640,11 @@
 }
 
 fn package_kind() -> Option<&'static str> {
+    if cfg!(target_os = "freebsd") {
+        // None of the package kinds the schema accepts describes a FreeBSD install, and the
+        // externally-managed message below is Flatpak-specific on other platforms.
+        return None;
+    }
     if opennow_core::update_apply::external_update_message().is_some() {
         return Some("flatpak");
     }
