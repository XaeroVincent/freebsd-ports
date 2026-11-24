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
@@ -40,6 +44,137 @@
 use crate::decoder::StreamInfo;
 use crate::Resolution;
 
+#[cfg(target_os = "freebsd")]
+use kqueue_poll::Epoll;
+
+/// FreeBSD has no epoll. A kqueue descriptor polls readable while any of its registered
+/// descriptors is readable, which is all the decoder needs from its event union.
+///
+/// Unlike epoll, kqueue keeps a registration queued once it has fired until `kevent` collects
+/// it, even if the descriptor has been drained since. Polling the raw descriptor would then
+/// wake up immediately forever, so handing the descriptor out first collects the queue: stale
+/// registrations are dropped, and a source that still has work stays readable (the filters are
+/// level-triggered).
+#[cfg(target_os = "freebsd")]
+mod kqueue_poll {
+    use std::os::fd::AsFd;
+    use std::os::fd::AsRawFd;
+    use std::os::fd::BorrowedFd;
+    use std::os::fd::FromRawFd;
+    use std::os::fd::OwnedFd;
+
+    use nix::errno::Errno;
+    use nix::libc;
+
+    pub struct Epoll(pub RefreshedFd);
+
+    pub struct RefreshedFd(OwnedFd);
+
+    impl RefreshedFd {
+        /// Re-evaluates the registrations so that readiness reflects what is pending now.
+        fn refresh(&self) {
+            let mut events: [libc::kevent; 2] = unsafe { std::mem::zeroed() };
+            let timeout = libc::timespec {
+                tv_sec: 0,
+                tv_nsec: 0,
+            };
+            // The collected events are not needed: the sources themselves are drained by the
+            // decoder. Errors only mean the descriptor is being torn down.
+            unsafe {
+                libc::kevent(
+                    self.0.as_raw_fd(),
+                    std::ptr::null(),
+                    0,
+                    events.as_mut_ptr(),
+                    events.len() as libc::c_int,
+                    &timeout,
+                );
+            }
+        }
+    }
+
+    impl AsFd for RefreshedFd {
+        fn as_fd(&self) -> BorrowedFd<'_> {
+            self.refresh();
+            self.0.as_fd()
+        }
+    }
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
+            Ok(Self(RefreshedFd(fd)))
+        }
+
+        pub fn add(&self, fd: BorrowedFd<'_>) -> Result<(), Errno> {
+            let mut change: libc::kevent = unsafe { std::mem::zeroed() };
+            change.ident = fd.as_raw_fd() as libc::uintptr_t;
+            change.filter = libc::EVFILT_READ;
+            change.flags = libc::EV_ADD;
+            let result = unsafe {
+                libc::kevent(
+                    (self.0).0.as_raw_fd(),
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
+
+    #[cfg(test)]
+    mod tests {
+        use super::*;
+
+        fn readable(fd: BorrowedFd<'_>) -> bool {
+            let mut poll = libc::pollfd {
+                fd: fd.as_raw_fd(),
+                events: libc::POLLIN,
+                revents: 0,
+            };
+            unsafe { libc::poll(&mut poll, 1, 0) > 0 }
+        }
+
+        #[test]
+        fn readiness_follows_the_registered_sources() {
+            let mut pipe = [0; 2];
+            assert_eq!(unsafe { libc::pipe(pipe.as_mut_ptr()) }, 0);
+            let (read_end, write_end) = unsafe {
+                (
+                    OwnedFd::from_raw_fd(pipe[0]),
+                    OwnedFd::from_raw_fd(pipe[1]),
+                )
+            };
+            let epoll = Epoll::new().unwrap();
+            epoll.add(read_end.as_fd()).unwrap();
+            assert!(!readable(epoll.0.as_fd()));
+            let byte = 1_u8;
+            unsafe { libc::write(write_end.as_raw_fd(), (&raw const byte).cast(), 1) };
+            assert!(readable(epoll.0.as_fd()));
+            // Still readable while the source has work, even across refreshes.
+            assert!(readable(epoll.0.as_fd()));
+            let mut sink = 0_u8;
+            unsafe { libc::read(read_end.as_raw_fd(), (&raw mut sink).cast(), 1) };
+            // Drained: the stale registration must not keep waking the poller.
+            assert!(!readable(epoll.0.as_fd()));
+        }
+    }
+}
+
 /// Error returned by `new_picture` methods of the backend, usually to indicate which kind of
 /// resource is needed before the picture can be successfully created.
 #[derive(Error, Debug)]
@@ -364,14 +499,29 @@
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
