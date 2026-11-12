--- opennow-qt/src/streaming/StreamVideoItemInput.cpp.orig
+++ opennow-qt/src/streaming/StreamVideoItemInput.cpp
@@ -504,7 +504,7 @@
 #if defined(Q_OS_WIN)
     return windowsGameplayVirtualKey(event->key(), event->modifiers(),
                                      event->nativeScanCode(), event->nativeVirtualKey());
-#elif defined(Q_OS_LINUX)
+#elif defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
     const auto evdevCode = PhysicalKeyMap::evdevCodeFromNativeScanCode(event->nativeScanCode());
     if (m_keyboardMap && (m_keyboardMap->locale == "sv-SE" || m_keyboardMap->locale == "de-DE")) {
         const auto physical = linuxPhysicalVirtualKey(event->nativeScanCode());
@@ -539,7 +539,7 @@
 {
     auto modifiers = event->modifiers();
     if (event->key() == Qt::Key_AltGr
-#if defined(Q_OS_WIN) || defined(Q_OS_LINUX)
+#if defined(Q_OS_WIN) || defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
         || (modifiers.testFlag(Qt::ControlModifier) && eventVirtualKey(event) == 0xa5)
 #endif
         || std::any_of(m_pressedKeys.cbegin(), m_pressedKeys.cend(),
