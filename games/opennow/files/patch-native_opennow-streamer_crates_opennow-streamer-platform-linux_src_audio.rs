--- native/opennow-streamer/crates/opennow-streamer-platform-linux/src/audio.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform-linux/src/audio.rs
@@ -599,7 +599,13 @@
                     std::thread::sleep(std::time::Duration::from_millis(2));
                     continue;
                 }
-                if written as c_int == -libc::ESTRPIPE {
+                // ESTRPIPE is Linux-specific. alsa-lib's non-Linux compatibility header defines
+                // it as ESPIPE, so that is what a suspended PCM returns on FreeBSD.
+                #[cfg(target_os = "freebsd")]
+                const ESTRPIPE: c_int = libc::ESPIPE;
+                #[cfg(not(target_os = "freebsd"))]
+                const ESTRPIPE: c_int = libc::ESTRPIPE;
+                if written as c_int == -ESTRPIPE {
                     return Err(Error::DeviceLost {
                         subsystem: Subsystem::Alsa,
                         reason: "PCM output was suspended".to_owned(),
