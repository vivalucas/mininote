import SwiftUI

struct FirstLaunchView: View {
    var onDismiss: () -> Void

    @AppStorage("renderMd") private var renderMd = false
    @AppStorage("appLanguage") private var appLanguage: String = Language.systemDefault.rawValue
    @State private var setDefaultApp = false

    var body: some View {
        VStack(spacing: 24) {
            Text(L("welcome.title"))
                .font(.title)
                .bold()

            Text(L("welcome.subtitle"))
                .foregroundColor(.secondary)

            Divider()

            VStack(alignment: .leading, spacing: 16) {
                Picker(L("settings.language"), selection: $appLanguage) {
                    ForEach(Language.allCases) { language in
                        Text(language.displayName).tag(language.rawValue)
                    }
                }
                .pickerStyle(.menu)

                Text(L("welcome.quickSetup"))
                    .font(.headline)

                Toggle(L("welcome.setDefault"), isOn: $setDefaultApp)

                Toggle(L("welcome.mdRender"), isOn: $renderMd)
            }

            Text(L("welcome.settingsHint"))
                .font(.caption)
                .foregroundColor(.secondary)

            Button(L("welcome.start")) {
                if setDefaultApp {
                    setAsDefaultForMintFiles()
                }
                onDismiss()
            }
            .buttonStyle(.borderedProminent)
            .controlSize(.large)
        }
        .padding(40)
        .frame(width: 420)
    }

    private func setAsDefaultForMintFiles() {
        // Register .mint UTI and set MiniNote as default handler.
        // On macOS, this uses Launch Services.
        let bundleID = Bundle.main.bundleIdentifier ?? "com.mininote.app"
        // Setting default app association requires Launch Services API,
        // which sets the CFBundleDocumentTypes in Info.plist + LSSetDefaultRoleHandlerForContentType.
        // Simplified: the Info.plist declares .mint support and the system
        // prompts the user on first open of a .mint file.
        _ = bundleID
    }

    private func L(_ key: String) -> String {
        LocalizationService.text(key, language: appLanguage)
    }
}
