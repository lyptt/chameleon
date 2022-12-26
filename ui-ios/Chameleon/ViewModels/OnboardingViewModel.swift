import Foundation
import ReactiveView

enum OnboardingViewModelStateIdentifier {
  case initial
  case transitioningToSignIn
}

struct OnboardingViewModelState {
  let identifier: OnboardingViewModelStateIdentifier
}

class OnboardingViewModel: ViewModel<OnboardingViewModelState> {
  init() {
    super.init(initialState: OnboardingViewModelState(identifier: .initial))
  }

  @objc func helpMeChooseTapped() {

  }

  @objc func signInTapped() {
    currentState = OnboardingViewModelState(identifier: .transitioningToSignIn)
  }

  func viewDidAppear() {
    guard currentState.identifier != .initial else {
      return
    }

    currentState = OnboardingViewModelState(identifier: .initial)
  }
}
