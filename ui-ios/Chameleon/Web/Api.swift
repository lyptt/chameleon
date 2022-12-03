import Foundation

protocol Api {
  var context: Context { get set }

  func serverStatus() async -> Result<Bool>
}
