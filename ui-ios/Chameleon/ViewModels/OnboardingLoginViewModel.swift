import Foundation
import ReactiveView

enum OnboardingLoginViewModelStateIdentifier {
  case initial
}

struct OnboardingLoginViewModelState {
  let identifier: OnboardingLoginViewModelStateIdentifier
}

class OnboardingLoginViewModel: ViewModel<OnboardingLoginViewModelState> {
  private let api: Api

  init(api: Api) {
    self.api = api
    super.init(initialState: OnboardingLoginViewModelState(identifier: .initial))
  }

  func viewDidAppear() {
    guard currentState.identifier != .initial else {
      return
    }

    currentState = OnboardingLoginViewModelState(identifier: .initial)
  }
}
