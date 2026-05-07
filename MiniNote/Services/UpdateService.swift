import Foundation

struct GitHubRelease: Codable {
    let tagName: String
    let htmlURL: URL
    let body: String?

    enum CodingKeys: String, CodingKey {
        case tagName = "tag_name"
        case htmlURL = "html_url"
        case body
    }
}

final class UpdateService: Sendable {
    private let repoOwner = "vivalucas"
    private let repoName = "mininote"

    var latestReleaseURL: URL {
        URL(string: "https://github.com/\(repoOwner)/\(repoName)/releases/latest")!
    }

    var repositoryURL: URL {
        URL(string: "https://github.com/\(repoOwner)/\(repoName)")!
    }

    var issuesURL: URL {
        URL(string: "https://github.com/\(repoOwner)/\(repoName)/issues")!
    }

    func checkForUpdate() async throws -> (hasUpdate: Bool, release: GitHubRelease?)? {
        let url = URL(
            string: "https://api.github.com/repos/\(repoOwner)/\(repoName)/releases/latest"
        )!
        var request = URLRequest(url: url)
        request.setValue("application/vnd.github+json", forHTTPHeaderField: "Accept")
        request.setValue("2022-11-28", forHTTPHeaderField: "X-GitHub-Api-Version")
        request.setValue("MiniNote/\(currentAppVersion())", forHTTPHeaderField: "User-Agent")

        let (data, response) = try await URLSession.shared.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200
        else { return nil }

        let release = try JSONDecoder().decode(GitHubRelease.self, from: data)
        let currentVersion = currentAppVersion()
        let latestVersion = normalizedVersion(release.tagName)

        let hasUpdate = isNewerVersion(latestVersion, than: currentVersion)
        return (hasUpdate, release)
    }

    func currentAppVersion() -> String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "0.0.0"
    }

    func normalizedVersion(_ version: String) -> String {
        version.hasPrefix("v") ? String(version.dropFirst()) : version
    }

    func isNewerVersion(_ remote: String, than local: String) -> Bool {
        let remoteParts = versionParts(remote)
        let localParts = versionParts(local)
        for index in 0..<max(remoteParts.count, localParts.count) {
            let remoteValue = index < remoteParts.count ? remoteParts[index] : 0
            let localValue = index < localParts.count ? localParts[index] : 0
            if remoteValue > localValue { return true }
            if remoteValue < localValue { return false }
        }
        return false
    }

    private func versionParts(_ version: String) -> [Int] {
        version
            .split(separator: ".")
            .map { part in
                let digits = part.prefix(while: { $0.isNumber })
                return Int(digits) ?? 0
            }
    }
}
