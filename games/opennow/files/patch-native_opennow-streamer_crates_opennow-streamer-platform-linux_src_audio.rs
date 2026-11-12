--- native/opennow-streamer/crates/opennow-streamer-platform-linux/src/audio.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform-linux/src/audio.rs
@@ -599,7 +599,12 @@
                     std::thread::sleep(std::time::Duration::from_millis(2));
                     continue;
                 }
-                if written as c_int == -libc::ESTRPIPE {
+                // ESTRPIPE is Linux-specific; alsa-lib reports "suspended" as -86 elsewhere.
+                #[cfg(target_os = "freebsd")]
+                const ESTRPIPE: c_int = 86;
+                #[cfg(not(target_os = "freebsd"))]
+                const ESTRPIPE: c_int = libc::ESTRPIPE;
+                if written as c_int == -ESTRPIPE {
                     return Err(Error::DeviceLost {
                         subsystem: Subsystem::Alsa,
                         reason: "PCM output was suspended".to_owned(),
