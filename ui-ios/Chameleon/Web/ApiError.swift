import Foundation

enum ApiError: Error {
  case invalidResponse
  case status(code: Int)
  case unknown
}
