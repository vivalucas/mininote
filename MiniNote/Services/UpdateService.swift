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

final class UpdateService {
    private let repoOwner = "vivalucas"
    private let repoName = "mininote"

    var latestReleaseURL: URL {
        URL(string: "https://github.com/\(repoOwner)/\(repoName)/releases/latest")!
    }

    func checkForUpdate() async throws -> (hasUpdate: Bool, release: GitHubRelease?)? {
        let url = URL(
            string: "https://api.github.com/repos/\(repoOwner)/\(repoName)/releases/latest"
        )!
        var request = URLRequest(url: url)
        request.setValue("application/vnd.github+json", forHTTPHeaderField: "Accept")
        request.setValue("2022-11-28", forHTTPHeaderField: "X-GitHub-Api-Version")

        let (data, response) = try await URLSession.shared.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200
        else { return nil }

        let release = try JSONDecoder().decode(GitHubRelease.self, from: data)
        let currentVersion = currentAppVersion()

        let hasUpdate = release.tagName.compare(currentVersion, options: .numeric) == .orderedDescending
        return (hasUpdate, release)
    }

    private func currentAppVersion() -> String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "0.0.0"
    }
}
