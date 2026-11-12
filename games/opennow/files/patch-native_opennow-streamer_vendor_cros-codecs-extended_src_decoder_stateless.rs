--- native/opennow-streamer/vendor/cros-codecs-extended/src/decoder/stateless.rs.orig
+++ native/opennow-streamer/vendor/cros-codecs-extended/src/decoder/stateless.rs
@@ -25,9 +25,13 @@
 use std::time::Duration;
 
 use nix::errno::Errno;
+#[cfg(not(target_os = "freebsd"))]
 use nix::sys::epoll::Epoll;
+#[cfg(not(target_os = "freebsd"))]
 use nix::sys::epoll::EpollCreateFlags;
+#[cfg(not(target_os = "freebsd"))]
 use nix::sys::epoll::EpollEvent;
+#[cfg(not(target_os = "freebsd"))]
 use nix::sys::epoll::EpollFlags;
 use nix::sys::eventfd::EventFd;
 use thiserror::Error;
@@ -40,6 +44,60 @@
 use crate::decoder::StreamInfo;
 use crate::Resolution;
 
+#[cfg(target_os = "freebsd")]
+use kqueue_poll::Epoll;
+
+/// FreeBSD has no epoll. A kqueue descriptor polls readable while any of its registered
+/// descriptors is readable, which is all the decoder needs from its event union.
+#[cfg(target_os = "freebsd")]
+mod kqueue_poll {
+    use std::os::fd::AsRawFd;
+    use std::os::fd::BorrowedFd;
+    use std::os::fd::FromRawFd;
+    use std::os::fd::OwnedFd;
+
+    use nix::errno::Errno;
+    use nix::libc;
+
+    pub struct Epoll(pub OwnedFd);
+
+    impl Epoll {
+        pub fn new() -> Result<Self, Errno> {
+            let raw = unsafe { libc::kqueue() };
+            if raw < 0 {
+                return Err(Errno::last());
+            }
+            let fd = unsafe { OwnedFd::from_raw_fd(raw) };
+            if unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
+                return Err(Errno::last());
+            }
+            Ok(Self(fd))
+        }
+
+        pub fn add(&self, fd: BorrowedFd<'_>) -> Result<(), Errno> {
+            let mut change: libc::kevent = unsafe { std::mem::zeroed() };
+            change.ident = fd.as_raw_fd() as libc::uintptr_t;
+            change.filter = libc::EVFILT_READ;
+            change.flags = libc::EV_ADD;
+            let result = unsafe {
+                libc::kevent(
+                    self.0.as_raw_fd(),
+                    &change,
+                    1,
+                    std::ptr::null_mut(),
+                    0,
+                    std::ptr::null(),
+                )
+            };
+            if result < 0 {
+                Err(Errno::last())
+            } else {
+                Ok(())
+            }
+        }
+    }
+}
+
 /// Error returned by `new_picture` methods of the backend, usually to indicate which kind of
 /// resource is needed before the picture can be successfully created.
 #[derive(Error, Debug)]
@@ -364,14 +422,29 @@
             ReadyFramesQueue::new().map_err(NewStatelessDecoderError::ReadyFramesQueue)?;
         let awaiting_format_event =
             EventFd::new().map_err(NewStatelessDecoderError::AwaitingFormatEventFd)?;
-        let epoll_fd =
-            Epoll::new(EpollCreateFlags::empty()).map_err(NewStatelessDecoderError::Epoll)?;
-        epoll_fd
-            .add(ready_queue.poll_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 1))
-            .map_err(NewStatelessDecoderError::EpollAdd)?;
-        epoll_fd
-            .add(awaiting_format_event.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 2))
-            .map_err(NewStatelessDecoderError::EpollAdd)?;
+        #[cfg(not(target_os = "freebsd"))]
+        let epoll_fd = {
+            let epoll_fd =
+                Epoll::new(EpollCreateFlags::empty()).map_err(NewStatelessDecoderError::Epoll)?;
+            epoll_fd
+                .add(ready_queue.poll_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 1))
+                .map_err(NewStatelessDecoderError::EpollAdd)?;
+            epoll_fd
+                .add(awaiting_format_event.as_fd(), EpollEvent::new(EpollFlags::EPOLLIN, 2))
+                .map_err(NewStatelessDecoderError::EpollAdd)?;
+            epoll_fd
+        };
+        #[cfg(target_os = "freebsd")]
+        let epoll_fd = {
+            let epoll_fd = Epoll::new().map_err(NewStatelessDecoderError::Epoll)?;
+            epoll_fd
+                .add(ready_queue.poll_fd())
+                .map_err(NewStatelessDecoderError::EpollAdd)?;
+            epoll_fd
+                .add(awaiting_format_event.as_fd())
+                .map_err(NewStatelessDecoderError::EpollAdd)?;
+            epoll_fd
+        };
 
         Ok(Self {
             backend,
