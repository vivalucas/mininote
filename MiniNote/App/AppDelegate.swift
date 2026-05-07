import AppKit

final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidResignActive(_ notification: Notification) {
        NotificationCenter.default.post(name: .appDidResignActive, object: nil)
    }

    func applicationWillTerminate(_ notification: Notification) {
        NotificationCenter.default.post(name: .appWillTerminate, object: nil)
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        NotificationCenter.default.post(name: .appWillTerminate, object: nil)
        // Brief delay to allow session flush to complete
        Thread.sleep(forTimeInterval: 0.1)
        return .terminateNow
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSWindow.allowsAutomaticWindowTabbing = false
    }

    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        if !flag {
            sender.windows.first?.makeKeyAndOrderFront(nil)
        }
        return true
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let appDidResignActive = Notification.Name("appDidResignActive")
    static let appWillTerminate = Notification.Name("appWillTerminate")
    static let editorZoomIn = Notification.Name("editorZoomIn")
    static let editorZoomOut = Notification.Name("editorZoomOut")
    static let editorZoomReset = Notification.Name("editorZoomReset")
    static let closeCurrentTab = Notification.Name("closeCurrentTab")
}
