import UIKit
import ReactiveView
import Cartography

protocol OnboardingViewControllerDelegate: AnyObject {
  func sessionLoaded(_ session: Session)
}

class OnboardingViewController: UIViewController {
  private let events = EventListenerRegistry()
  private let vm: OnboardingViewModel

  private let titleLabel = UILabel()
  private let bodyLabel = UILabel()
  private let mascotImage = UIImageView()
  private let newCommunityButton = BigButton(style: .alternate)
  private let existingCommunityButton = BigButton(style: .regular)

  weak var delegate: OnboardingViewControllerDelegate?

  init(viewModel: OnboardingViewModel) {
    self.vm = viewModel
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func loadView() {
    super.loadView()
    self.view.backgroundColor = .systemBackground

    title = NSLocalizedString("Chameleon", comment: "Screen: Title: SessionTransition")
    let titleView = TitleView(frame: CGRect(x: 0, y: 0, width: 9001, height: 42))
    navigationItem.titleView = titleView

    [mascotImage, titleLabel, bodyLabel, newCommunityButton, existingCommunityButton].forEach { view.addSubview($0) }

    titleLabel.font = .preferredFont(forTextStyle: .title1)
    bodyLabel.font = .preferredFont(forTextStyle: .body)
    mascotImage.contentMode = .scaleAspectFit
    mascotImage.image = UIImage(named: "Chamille")
    mascotImage.setContentHuggingPriority(.defaultHigh, for: .vertical)
    mascotImage.setContentCompressionResistancePriority(.defaultLow, for: .vertical)
    let mascotImageAspectRatio = mascotImage.image!.size.width / mascotImage.image!.size.height
    titleLabel.text = NSLocalizedString("Welcome to Chameleon", comment: "Screen: Onboarding: Title")
    titleLabel.lineBreakMode = .byWordWrapping
    titleLabel.numberOfLines = 0
    bodyLabel.text = NSLocalizedString("Chameleon is a community run social network, giving you control over your data.\n\nIf you don't have an account on a Chameleon community yet, we can help you choose one.", comment: "Screen: Onboarding: Title")
    bodyLabel.lineBreakMode = .byWordWrapping
    bodyLabel.numberOfLines = 0
    newCommunityButton.setTitle(NSLocalizedString("Help me choose", comment: "Screen: Onboarding: Help me choose button"), for: .normal)
    existingCommunityButton.setTitle(NSLocalizedString("Sign in", comment: "Screen: Onboarding: Sign in button"), for: .normal)

    newCommunityButton.addTarget(vm, action: #selector(OnboardingViewModel.helpMeChooseTapped), for: .touchUpInside)
    existingCommunityButton.addTarget(vm, action: #selector(OnboardingViewModel.signInTapped), for: .touchUpInside)

    constrain(view, titleLabel, bodyLabel, mascotImage, existingCommunityButton, newCommunityButton) { container, title, body, mascot, existingCommunityButton, newCommunityButton in
      title.top == container.safeAreaLayoutGuide.top + 20
      title.leading == container.safeAreaLayoutGuide.leading + 20
      title.trailing == container.safeAreaLayoutGuide.trailing - 20

      body.top == title.bottom + 20
      body.leading == title.leading
      body.trailing == title.trailing

      mascot.leading == body.leading + 50
      mascot.trailing == body.trailing - 50
      mascot.centerY == container.centerY + 50
      mascot.height == mascot.width * mascotImageAspectRatio

      existingCommunityButton.leading == title.leading
      existingCommunityButton.trailing == title.trailing
      existingCommunityButton.bottom == container.safeAreaLayoutGuide.bottom

      newCommunityButton.leading == title.leading
      newCommunityButton.trailing == title.trailing
      newCommunityButton.bottom == existingCommunityButton.top - 20
    }
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    title = NSLocalizedString("Chameleon", comment: "Screen: Title: Onboarding")

    events <~ vm.on("statechange") { [weak self] event in
      guard let current = event.data["current"] as? OnboardingViewModelState else { return }
      let previous = event.data["previous"] as? OnboardingViewModelState ?? current
      self?.handleTransition(from: previous, to: current)
    }
  }

  override func viewDidAppear(_ animated: Bool) {
    super.viewDidAppear(animated)
    vm.viewDidAppear()
  }

  private func handleTransition(from: OnboardingViewModelState, to: OnboardingViewModelState) {
    switch to.identifier {
    case .initial:
      break
    case .transitioningToSignIn:
      let vc = container.resolve(OnboardingLoginViewController.self)!
      vc.delegate = delegate
      navigationController?.pushViewController(vc, animated: true)
      break
    }
  }
}
