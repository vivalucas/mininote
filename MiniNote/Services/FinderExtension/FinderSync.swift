import AppKit
import FinderSync

final class FinderSyncExtension: FIFinderSync {
    private let creator = FinderMintFileCreator()

    override init() {
        super.init()
        FIFinderSyncController.default().directoryURLs = [
            URL(fileURLWithPath: NSHomeDirectory())
        ]
    }

    override var toolbarItemName: String {
        "MiniNote"
    }

    override var toolbarItemToolTip: String {
        "Create a MiniNote document"
    }

    override var toolbarItemImage: NSImage {
        NSImage(named: NSImage.Name("AppIcon")) ?? NSImage()
    }

    override func menu(for menuKind: FIMenuKind) -> NSMenu? {
        let menu = NSMenu(title: "MiniNote")
        let item = NSMenuItem(
            title: "New MiniNote Document",
            action: #selector(createMiniNoteDocument(_:)),
            keyEquivalent: ""
        )
        item.target = self
        menu.addItem(item)
        return menu
    }

    @objc private func createMiniNoteDocument(_ sender: Any?) {
        guard let directory = selectedDirectory else { return }
        creator.createNewFile(in: directory)
    }

    private var selectedDirectory: URL? {
        let controller = FIFinderSyncController.default()
        if let target = controller.targetedURL() {
            return target.hasDirectoryPath ? target : target.deletingLastPathComponent()
        }
        if let selected = controller.selectedItemURLs()?.first {
            return selected.hasDirectoryPath ? selected : selected.deletingLastPathComponent()
        }
        return nil
    }
}

final class FinderMintFileCreator {
    func createNewFile(in directory: URL) {
        let fileName = availableFileName(
            baseName: "Untitled.mint",
            in: directory
        )
        let fileURL = directory.appendingPathComponent(fileName)

        // Create an empty .mint file
        let content = ""
        try? content.write(to: fileURL, atomically: true, encoding: .utf8)

        // Reveal and select the new file in Finder
        NSWorkspace.shared.activateFileViewerSelecting([fileURL])
    }

    private func availableFileName(baseName: String, in directory: URL) -> String {
        let fileManager = FileManager.default
        let originalURL = directory.appendingPathComponent(baseName)
        guard fileManager.fileExists(atPath: originalURL.path) else {
            return baseName
        }

        let ext = originalURL.pathExtension
        let stem = originalURL.deletingPathExtension().lastPathComponent
        var counter = 2
        while true {
            let candidate = "\(stem) \(counter).\(ext)"
            let candidateURL = directory.appendingPathComponent(candidate)
            if !fileManager.fileExists(atPath: candidateURL.path) {
                return candidate
            }
            counter += 1
        }
    }
}
