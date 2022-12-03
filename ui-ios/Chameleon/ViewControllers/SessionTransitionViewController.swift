import UIKit
import ReactiveView
import Cartography

class SessionTransitionViewController: UIViewController, OnboardingViewControllerDelegate {
  private let events = EventListenerRegistry()
  private let vm: SessionTransitionViewModel
  private var transitionType: SessionTransition?

  private let progressView = UIActivityIndicatorView(style: .large)

  init(viewModel: SessionTransitionViewModel) {
    self.vm = viewModel
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func loadView() {
    super.loadView()
    self.view.backgroundColor = .systemBackground

    [progressView].forEach { view.addSubview($0) }

    progressView.alpha = 0
    progressView.hidesWhenStopped = false
    progressView.startAnimating()

    constrain(view, progressView) { container, progressView in
      progressView.center == container.center
    }
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    title = NSLocalizedString("Chameleon", comment: "Screen: Title: SessionTransition")
    let titleView = TitleView(frame: CGRect(x: 0, y: 0, width: 9001, height: 42))
    navigationItem.titleView = titleView

    events <~ vm.on("statechange") { [weak self] event in
      guard let current = event.data["current"] as? SessionTransitionViewModelState else { return }
      let previous = event.data["previous"] as? SessionTransitionViewModelState ?? current
      self?.handleTransition(from: previous, to: current)
    }

    if let transitionType = transitionType {
      vm.transition(to: transitionType)
    }
  }

  private func handleTransition(from: SessionTransitionViewModelState, to: SessionTransitionViewModelState) {
    switch to.identifier {
    case .initial:
      UIView.animate(withDuration: 0.25, delay: 3) { [weak self] in
        self?.progressView.alpha = 1
      }
    case .sessionLoaded(let session):
      presentedViewController?.dismiss(animated: true)
      UIView.animate(withDuration: 0.15, animations: { [weak self] in
        self?.progressView.alpha = 0
      }) { [weak self] _ in
        guard let window = UIApplication.currentWindow, let vc = self?.container.resolve(MainViewController.self) else {
          return
        }

        vc.session = session
        window.rootViewController = vc
      }
    case .sessionLoadFailed:
      let vc = container.resolve(OnboardingViewController.self)!
      present(vc, animated: true)
      break
    case .noSessionAvailable:
      let vc = container.resolve(OnboardingViewController.self)!
      vc.delegate = self
      let nvc = UINavigationController(rootViewController: vc)
      nvc.isModalInPresentation = true
      present(nvc, animated: true)
    }
  }

  /// Transitions the currently loaded session to the specified type.
  /// > Note: Should only be called immediately after initialization. Any calls after this view controller's view has loaded
  /// > will have no effect.
  func transition(to type: SessionTransition) {
    transitionType = type
  }

  func sessionLoaded(_ session: Session) {
    vm.sessionLoaded(session)
  }
}
