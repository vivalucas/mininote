import Foundation

struct SessionTabInfo: Codable {
    let id: UUID
    let type: DocumentType
    let fileURL: URL?
    let cursorPosition: Int
    let isRendering: Bool
}

struct SessionData: Codable {
    var tabs: [SessionTabInfo]
    var activeTabIndex: Int
}

actor SessionStore {
    private let sessionsDir: URL
    private let sessionFile: URL
    private var writeTask: Task<Void, Never>?
    /// Scratch doc content waiting to be persisted after debounce.
    private var pendingContent: [UUID: String] = [:]

    init() {
        let appSupport = FileManager.default.urls(
            for: .applicationSupportDirectory,
            in: .userDomainMask
        ).first!
        sessionsDir = appSupport
            .appendingPathComponent("MiniNote")
            .appendingPathComponent("sessions")
        sessionFile = sessionsDir.appendingPathComponent("session.json")
        try? FileManager.default.createDirectory(
            at: sessionsDir,
            withIntermediateDirectories: true
        )
    }

    // MARK: - Session metadata

    func saveSession(tabs: [Document], activeIndex: Int) {
        let info: [SessionTabInfo] = tabs.map { doc in
            SessionTabInfo(
                id: doc.id,
                type: doc.type,
                fileURL: doc.fileURL,
                cursorPosition: doc.cursorPosition,
                isRendering: doc.isRendering
            )
        }
        let data = SessionData(tabs: info, activeTabIndex: activeIndex)
        if let encoded = try? JSONEncoder().encode(data) {
            try? encoded.write(to: sessionFile, options: .atomic)
        }
    }

    func loadSession() -> (tabs: [SessionTabInfo], activeIndex: Int)? {
        guard let data = try? Data(contentsOf: sessionFile),
              let session = try? JSONDecoder().decode(SessionData.self, from: data)
        else { return nil }
        return (session.tabs, session.activeTabIndex)
    }

    // MARK: - Scratch document auto-save (debounced + lifecycle)

    /// Called on every content change for a scratch document.
    /// Stores content in memory and (re)schedules a 3-second debounced write.
    func scheduleScratchWrite(id: UUID, content: String) {
        pendingContent[id] = content
        writeTask?.cancel()
        writeTask = Task { [pendingContent] in
            try? await Task.sleep(for: .seconds(3))
            guard !Task.isCancelled else { return }
            for (docId, text) in pendingContent {
                writeScratchContent(id: docId, content: text)
            }
        }
    }

    /// Immediate write for a single scratch document (tab switch, app resign).
    func flushImmediately(id: UUID, content: String) {
        writeTask?.cancel()
        writeTask = nil
        writeScratchContent(id: id, content: content)
        pendingContent.removeValue(forKey: id)
    }

    /// Flush all pending scratch documents to disk (app terminate, background).
    func flushAll(scratchDocs: [(id: UUID, content: String)]) {
        writeTask?.cancel()
        writeTask = nil
        for (id, content) in scratchDocs {
            writeScratchContent(id: id, content: content)
        }
        pendingContent.removeAll()
    }

    // MARK: - Individual file I/O

    func writeScratchContent(id: UUID, content: String) {
        let fileURL = scratchFileURL(for: id)
        try? content.write(to: fileURL, atomically: true, encoding: .utf8)
    }

    func readScratchContent(id: UUID) -> String? {
        let fileURL = scratchFileURL(for: id)
        return try? String(contentsOf: fileURL, encoding: .utf8)
    }

    func deleteScratchFile(id: UUID) {
        try? FileManager.default.removeItem(at: scratchFileURL(for: id))
    }

    func scratchFileURL(for id: UUID) -> URL {
        sessionsDir.appendingPathComponent("\(id.uuidString).mint")
    }
}
