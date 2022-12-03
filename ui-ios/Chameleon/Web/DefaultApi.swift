import Foundation

class DefaultApi: Api {
  private let session = {
    let config = URLSessionConfiguration.default
    return URLSession(configuration: config)
  }()
  private let decoder = JSONDecoder()

  var context = Context(apiUrl: URL(string:"http://localhost:8000")!, cdnUrl: URL(string:"http://localhost:8000")!, name: "Stub")

  func serverStatus() async -> Result<Bool> {
    do {
      let req = URLRequest(url: context.apiUrl.appending(path: ".well-known/status"))
      let (data, res) = try await session.data(for: req)
      guard let response = res as? HTTPURLResponse else {
        throw ApiError.invalidResponse
      }

      guard response.statusCode == 200 else {
        throw ApiError.status(code: response.statusCode)
      }

      let body = try decoder.decode(ApiStatus.self, from: data)

      return .ok(data: body.db.ok)
    } catch {
      if let err = error as? ApiError {
        return .error(err: .api(err: err))
      }

      return .error(err: .unknown)
    }
  }
}
