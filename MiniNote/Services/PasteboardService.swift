import AppKit

final class PasteboardService {
    /// Returns the plain text string from the general pasteboard, stripping all formatting.
    func plainTextFromPasteboard() -> String? {
        let pasteboard = NSPasteboard.general
        return pasteboard.string(forType: .string)
    }

    /// Copy plain text to pasteboard.
    func copyToPasteboard(_ text: String) {
        let pasteboard = NSPasteboard.general
        pasteboard.clearContents()
        pasteboard.setString(text, forType: .string)
    }
}
