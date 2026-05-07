import Foundation
import Observation

@Observable
final class Document: Identifiable {
    let id: UUID
    var content: String
    var type: DocumentType
    var fileURL: URL?
    var cursorPosition: Int
    var isModified: Bool
    var isRendering: Bool

    var isScratch: Bool { fileURL == nil }

    var fileName: String {
        displayName()
    }

    func displayName(language: String? = nil) -> String {
        fileURL?.lastPathComponent ?? LocalizationService.text(
            "common.untitled",
            language: language
        )
    }

    var encoding: String.Encoding = .utf8
    var lineEnding: LineEnding = .lf

    init(
        id: UUID = UUID(),
        content: String = "",
        type: DocumentType = .mint,
        fileURL: URL? = nil,
        cursorPosition: Int = 0,
        isModified: Bool = false,
        isRendering: Bool = false
    ) {
        self.id = id
        self.content = content
        self.type = type
        self.fileURL = fileURL
        self.cursorPosition = cursorPosition
        self.isModified = isModified
        self.isRendering = isRendering
    }
}

enum LineEnding: String, CaseIterable {
    case lf   = "LF"
    case crlf = "CRLF"

    var characters: String {
        switch self {
        case .lf:   "\n"
        case .crlf: "\r\n"
        }
    }
}
