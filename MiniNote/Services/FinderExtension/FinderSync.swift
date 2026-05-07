import AppKit

final class FinderSync: NSObject {
    /// Entry point for Finder Extension — called when user right-clicks in Finder.
    /// Registers a service that creates a new .mint file in the targeted directory.
    func handleCreateNewFile(in directory: URL) {
        let fileService = FileService()
        let fileName = fileService.defaultFileName(type: .mint)
        let fileURL = directory.appendingPathComponent(fileName)

        // Create an empty .mint file
        let content = ""
        try? content.write(to: fileURL, atomically: true, encoding: .utf8)

        // Reveal and select the new file in Finder
        NSWorkspace.shared.activateFileViewerSelecting([fileURL])
    }
}
