import Foundation

struct ApiStatusComponent: Decodable {
  let ok: Bool
  let error: String?
  let updated_at: Int64
}

struct ApiStatus: Decodable {
  let db: ApiStatusComponent
}
