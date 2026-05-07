import SwiftUI
import Observation

@Observable
final class AppState {
    var documents: [Document] = []
    var activeTabId: UUID = UUID()
    var cursorPosition: Int = 0

    var activeDocument: Document? {
        documents.first(where: { $0.id == activeTabId })
    }

    private let fileService = FileService()
    private let sessionStore = SessionStore()

    // MARK: - Tab Management

    func newTab() {
        let doc = Document(isRendering: defaultRender(for: .mint))
        documents.append(doc)
        activeTabId = doc.id
        saveSessionSnapshot()
    }

    func closeTab(_ doc: Document) {
        if doc.isScratch {
            Task { await sessionStore.deleteScratchFile(id: doc.id) }
        }
        documents.removeAll { $0.id == doc.id }
        if activeTabId == doc.id {
            activeTabId = documents.first?.id ?? UUID()
        }
        if documents.isEmpty {
            newTab()
        }
        saveSessionSnapshot()
    }

    func switchToTab(_ id: UUID) {
        guard id != activeTabId else { return }
        if let current = activeDocument, current.isScratch {
            Task {
                await sessionStore.flushImmediately(id: current.id, content: current.content)
            }
        }
        activeTabId = id
        if let target = documents.first(where: { $0.id == id }) {
            cursorPosition = target.cursorPosition
        }
        saveSessionSnapshot()
    }

    // MARK: - Markdown Toggle

    func toggleRendering(for doc: Document) {
        doc.isRendering.toggle()
    }

    // MARK: - File Operations

    func openFile() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText, .init(filenameExtension: "mint")!]
        panel.allowsMultipleSelection = false
        panel.begin { [weak self] response in
            guard let self, response == .OK, let url = panel.url else { return }
            if let existing = self.documents.first(where: { $0.fileURL == url }) {
                self.activeTabId = existing.id
                return
            }
            if let doc = try? self.fileService.open(url: url) {
                doc.isRendering = self.defaultRender(for: doc.type)
                self.documents.append(doc)
                self.activeTabId = doc.id
                self.saveSessionSnapshot()
            }
        }
    }

    func saveDocument(_ doc: Document) {
        if doc.fileURL != nil {
            try? fileService.save(document: doc)
        } else {
            saveAsDocument(doc)
        }
    }

    func saveAsDocument(_ doc: Document) {
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.plainText, .init(filenameExtension: "mint")!]
        panel.nameFieldStringValue = doc.fileName
        panel.begin { [weak self] response in
            guard let self, response == .OK, let url = panel.url else { return }
            try? self.fileService.saveAs(document: doc, to: url)
        }
    }

    // MARK: - Zoom

    func zoomIn() {
        // Handled by NSTextView via font size changes.
        // Post notification that EditorTextView can observe.
        NotificationCenter.default.post(name: .editorZoomIn, object: nil)
    }

    func zoomOut() {
        NotificationCenter.default.post(name: .editorZoomOut, object: nil)
    }

    func resetZoom() {
        NotificationCenter.default.post(name: .editorZoomReset, object: nil)
    }

    // MARK: - Session Persistence

    func scheduleScratchSave(for doc: Document) {
        guard doc.isScratch else { return }
        Task {
            await sessionStore.scheduleScratchWrite(id: doc.id, content: doc.content)
        }
    }

    func flushCurrentScratch() {
        guard let doc = activeDocument, doc.isScratch else { return }
        Task {
            await sessionStore.flushImmediately(id: doc.id, content: doc.content)
        }
    }

    func flushAllScratch() {
        let scratchDocs = documents.filter { $0.isScratch }.map {
            (id: $0.id, content: $0.content)
        }
        Task {
            await sessionStore.flushAll(scratchDocs: scratchDocs)
        }
    }

    func saveSessionSnapshot() {
        let activeIndex = documents.firstIndex(where: { $0.id == activeTabId }) ?? 0
        Task { await sessionStore.saveSession(tabs: documents, activeIndex: activeIndex) }
    }

    func restoreSessionOrStartFresh() {
        let behavior = UserDefaults.standard.string(forKey: "startupBehavior") ?? "continue"
        guard behavior == "continue" else {
            Task { @MainActor in newTab() }
            return
        }
        Task {
            if let (tabInfos, activeIndex) = await sessionStore.loadSession(),
               !tabInfos.isEmpty {
                var restored: [Document] = []
                for info in tabInfos {
                    let doc = Document(
                        id: info.id,
                        type: info.type,
                        fileURL: info.fileURL,
                        cursorPosition: info.cursorPosition,
                        isRendering: info.isRendering
                    )
                    if doc.isScratch {
                        doc.content = await sessionStore.readScratchContent(id: info.id) ?? ""
                    } else if let url = info.fileURL {
                        doc.content = (try? String(contentsOf: url, encoding: .utf8)) ?? ""
                    }
                    restored.append(doc)
                }
                await MainActor.run {
                    documents = restored
                    let idx = min(activeIndex, restored.count - 1)
                    activeTabId = restored[idx].id
                    cursorPosition = restored[idx].cursorPosition
                }
            } else {
                await MainActor.run { newTab() }
            }
        }
    }

    // MARK: - Lifecycle

    func onLifecycleEvent(_ notification: Notification) {
        switch notification.name {
        case .appDidResignActive:
            flushCurrentScratch()
            saveSessionSnapshot()
        case .appWillTerminate:
            flushAllScratch()
            saveSessionSnapshot()
        default:
            break
        }
    }

    // MARK: - Settings Helpers

    private func defaultRender(for type: DocumentType) -> Bool {
        let key: String = switch type {
        case .mint: "renderMint"
        case .txt:  "renderTxt"
        case .md:   "renderMd"
        }
        return UserDefaults.standard.bool(forKey: key)
    }
}
