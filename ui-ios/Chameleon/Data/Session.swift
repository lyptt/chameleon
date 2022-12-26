import Foundation
import KeychainSwift

struct Session: Codable {
  let id: UUID
  let serverUrl: URL
  let serverName: String
  let serverApiUrl: URL
  let serverCdnUrl: URL
  let sessionId: UUID
  let userId: UUID
  let appId: UUID
  let createdAt: Date
  let updatedAt: Date
  let accessExpiresAt: Date
  let refreshExpiresAt: Date

  func save() async -> Result<Void> {
    do {
      let encoder = JSONEncoder()
      let data = try encoder.encode(self)
      guard let body = String(data: data, encoding: .utf8) else {
        throw CoreError.stringEncode
      }

      let keychain = KeychainSwift(keyPrefix: "uk.lyptt.chameleon")
      keychain.set(body, forKey: id.uuidString)

      UserDefaults.standard.set(id.uuidString, forKey: "last-session")

      return .ok(data: ())
    } catch {
      if let err = error as? CoreError {
        return .error(err: err)
      }

      return .error(err: .unknown)
    }
  }

  static func load() async -> Result<Session> {
    guard let lastSession = UserDefaults.standard.string(forKey: "last-session") else {
      return .error(err: .missingData)
    }

    return await load(id: lastSession)
  }

  static func load(id: String) async -> Result<Session> {
    do {
      let keychain = KeychainSwift(keyPrefix: "uk.lyptt.chameleon")
      let decoder = JSONDecoder()

      guard let body = keychain.get(id) else {
        throw CoreError.missingData
      }

      guard let data = body.data(using: .utf8) else {
        throw CoreError.stringParse
      }

      guard let ret = try? decoder.decode(Session.self, from: data) else {
        throw CoreError.jsonParse
      }

      return .ok(data: ret)
    } catch {
      if let err = error as? CoreError {
        return .error(err: err)
      }

      return .error(err: .unknown)
    }
  }
}
