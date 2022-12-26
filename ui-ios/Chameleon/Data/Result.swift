import Foundation

enum CoreError: Error {
  case api(err: ApiError)
  case unknown
  case stringParse
  case stringEncode
  case jsonParse
  case jsonEncode
  case missingData
}

enum Result<T> {
  case ok(data: T)
  case error(err: CoreError)
}

