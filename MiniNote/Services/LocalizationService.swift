import Foundation

enum Language: String, CaseIterable, Identifiable {
    case chinese = "zh"
    case english = "en"
    case japanese = "ja"
    case german = "de"
    case french = "fr"
    case spanish = "es"
    case traditionalChinese = "zh-Hant"
    case portugueseBrazil = "pt-BR"
    case italian = "it"
    case russian = "ru"

    var id: String { rawValue }

    var displayName: String {
        switch self {
        case .chinese: "中文"
        case .english: "English"
        case .japanese: "日本語"
        case .german: "Deutsch"
        case .french: "Français"
        case .spanish: "Español"
        case .traditionalChinese: "繁體中文"
        case .portugueseBrazil: "Português"
        case .italian: "Italiano"
        case .russian: "Русский"
        }
    }

    static var systemDefault: Language {
        let code = Locale.current.language.languageCode?.identifier ?? "en"
        let script = Locale.current.language.script?.identifier ?? ""
        switch code {
        case "zh":
            if script == "Hant" { return .traditionalChinese }
            return .chinese
        case "ja":
            return .japanese
        case "de":
            return .german
        case "fr":
            return .french
        case "es":
            return .spanish
        case "pt":
            return .portugueseBrazil
        case "it":
            return .italian
        case "ru":
            return .russian
        default:
            return .english
        }
    }
}

enum LocalizationService {
    static func text(_ key: String, language rawValue: String? = nil) -> String {
        return NSLocalizedString(key, comment: "")
    }

    static func formatted(_ key: String, _ arguments: CVarArg..., language: String? = nil) -> String {
        return String(format: NSLocalizedString(key, comment: ""), arguments: arguments)
    }
}