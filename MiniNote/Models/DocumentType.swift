import Foundation

enum DocumentType: String, CaseIterable, Codable, Sendable {
    case mint
    case txt
    case md

    var displayName: String {
        switch self {
        case .mint: String(localized: "document.mint")
        case .txt:  String(localized: "document.txt")
        case .md:   String(localized: "document.md")
        }
    }

    var fileExtension: String { rawValue }

    var defaultRenderSetting: Bool { false }
}
