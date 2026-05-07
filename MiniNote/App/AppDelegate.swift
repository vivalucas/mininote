import AppKit

final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidResignActive(_ notification: Notification) {
        // App lost focus → flush all scratch documents
        NotificationCenter.default.post(name: .appDidResignActive, object: nil)
    }

    func applicationWillTerminate(_ notification: Notification) {
        // App quitting → flush all scratch documents
        NotificationCenter.default.post(name: .appWillTerminate, object: nil)
    }

    func applicationDidChangeScreenParameters(_ notification: Notification) {
        // Screen sleep / lid close detected
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        // Give ContentView time to flush session data
        NotificationCenter.default.post(name: .appWillTerminate, object: nil)
        return .terminateNow
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        // Disable default macOS tabbing behavior in favor of our custom tabs
        NSWindow.allowsAutomaticWindowTabbing = false
    }

    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        if !flag {
            // Bring back main window if all windows were closed
            sender.windows.first?.makeKeyAndOrderFront(nil)
        }
        return true
    }
}

extension Notification.Name {
    static let appDidResignActive = Notification.Name("appDidResignActive")
    static let appWillTerminate = Notification.Name("appWillTerminate")
}
