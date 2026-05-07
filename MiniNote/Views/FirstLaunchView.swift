import SwiftUI

struct FirstLaunchView: View {
    var onDismiss: () -> Void

    @AppStorage("renderMd") private var renderMd = false
    @State private var setDefaultApp = false

    var body: some View {
        VStack(spacing: 24) {
            Text("欢迎使用 MiniNote")
                .font(.title)
                .bold()

            Text("一个轻量级纯文本编辑器，对标 Windows 11 新版记事本。")
                .foregroundColor(.secondary)

            Divider()

            VStack(alignment: .leading, spacing: 16) {
                Text("快速设置")
                    .font(.headline)

                Toggle("将 .mint 文件默认用 MiniNote 打开", isOn: $setDefaultApp)

                Toggle("打开 .md 文件时默认渲染 Markdown", isOn: $renderMd)
            }

            Text("随时可在偏好设置（Cmd+,）中修改这些选项。")
                .font(.caption)
                .foregroundColor(.secondary)

            Button("开始使用") {
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
    }
}
