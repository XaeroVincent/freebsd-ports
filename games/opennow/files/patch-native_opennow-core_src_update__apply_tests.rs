--- native/opennow-core/src/update_apply/tests.rs.orig
+++ native/opennow-core/src/update_apply/tests.rs
@@ -18,6 +18,20 @@
     assert!(validate_linux_install_kind(InstallKind::WindowsMsi, None).is_err());
 }
 
+#[cfg(target_os = "freebsd")]
+#[test]
+fn freebsd_packages_are_updated_through_pkg() {
+    assert!(external_update_message().is_some());
+    assert!(compatible_package_extension().is_err());
+    assert!(
+        detect_install_kind(
+            Path::new("/usr/local/bin/opennow-qt"),
+            Path::new("OpenNOW-Qt-1.0.0-FreeBSD-x64.txz"),
+        )
+        .is_err()
+    );
+}
+
 #[test]
 fn windows_zip_is_selected_until_the_install_is_msi_registered() {
     assert_eq!(windows_update_package_extension(false).unwrap(), "zip");
