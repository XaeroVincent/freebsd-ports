--- native/opennow-core/src/updater.rs.orig
+++ native/opennow-core/src/updater.rs
@@ -1937,6 +1937,18 @@
     }
 
     #[test]
+    fn freebsd_assets_are_never_selected_for_native_updates() {
+        // FreeBSD is updated through pkg(8); a .txz release asset must not be advertised.
+        let name = "OpenNOW-Qt-1.0.0-FreeBSD-x64.txz".to_owned();
+        let assets = [Asset {
+            browser_download_url: format!("{RELEASE_ASSET_PREFIX}v1.0.0/{name}"),
+            name,
+            size: 123,
+        }];
+        assert!(compatible_asset_for_target(&assets, "freebsd", "x86_64", "txz").is_none());
+    }
+
+    #[test]
     fn darwin_manifests_bind_the_exact_architecture() {
         let signing = SigningKey::from_bytes(&[7_u8; 32]);
         for (architecture, other) in [("x64", "arm64"), ("arm64", "x64")] {
