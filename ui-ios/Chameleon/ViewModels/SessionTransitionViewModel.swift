import Foundation
import ReactiveView

enum SessionTransition {
  case initialize
  case signedOut
  case session(id: String)
}

enum SessionTransitionViewModelStateIdentifier {
  case initial
  case sessionLoaded(session: Session)
  case sessionLoadFailed(error: CoreError)
  case noSessionAvailable
}

struct SessionTransitionViewModelState {
  let identifier: SessionTransitionViewModelStateIdentifier
}

class SessionTransitionViewModel: ViewModel<SessionTransitionViewModelState> {
  private let api: Api

  init(api: Api) {
    self.api = api
    super.init(initialState: SessionTransitionViewModelState(identifier: .initial))
  }

  func transition(to type: SessionTransition) {
    switch type {
    case .initialize:
      Task {
        switch await Session.load() {
        case .ok(data: let data):
          currentState = SessionTransitionViewModelState(identifier: .sessionLoaded(session: data))
        case .error(err: let err):
          switch err {
          case .missingData:
            currentState = SessionTransitionViewModelState(identifier: .noSessionAvailable)
          default:
            currentState = SessionTransitionViewModelState(identifier: .sessionLoadFailed(error: err))
          }
        }
      }
    case .signedOut:
      currentState = SessionTransitionViewModelState(identifier: .initial)
    case .session(id: let id):
      Task {
        switch await Session.load(id: id) {
        case .ok(data: let data):
          currentState = SessionTransitionViewModelState(identifier: .sessionLoaded(session: data))
        case .error(err: let err):
          switch err {
          case .missingData:
            // TODO: If a session is spontaneously unavailable, we should show the option to pick another available account
            currentState = SessionTransitionViewModelState(identifier: .noSessionAvailable)
          default:
            currentState = SessionTransitionViewModelState(identifier: .sessionLoadFailed(error: err))
          }
        }
      }
    }
  }

  func sessionLoaded(_ session: Session) {
    Task {
      // TODO: How do we handle errors here?
      let _ = await session.save()
      currentState = SessionTransitionViewModelState(identifier: .sessionLoaded(session: session))
    }
  }
}
