--- native/opennow-core/src/update_apply/mod.rs.orig	2026-11-08 13:59:09.000000000 -0800
+++ native/opennow-core/src/update_apply/mod.rs	2026-11-08 18:51:58.143622000 -0800
@@ -232,6 +232,10 @@ pub fn compatible_package_extension() -> Result<&'stat
         let executable = canonical_file(Path::new(&executable))?;
         let root = installation_target(InstallKind::WindowsMsi, &executable)?;
         windows_update_package_extension(managed::windows_managed(&root)?)
+    }
+    #[cfg(target_os = "freebsd")]
+    {
+	Ok("txz")
     }
 }
 
