import Foundation

enum DocumentType: String, CaseIterable, Codable {
    case mint
    case txt
    case md

    var displayName: String {
        switch self {
        case .mint: LocalizationService.text("document.mint")
        case .txt:  LocalizationService.text("document.txt")
        case .md:   LocalizationService.text("document.md")
        }
    }

    var fileExtension: String { rawValue }

    var defaultRenderSetting: Bool { false }
}
