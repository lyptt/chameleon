import Foundation
import UIKit
import ReactiveView
import Cartography

class OnboardingLoginViewController: UIViewController {
  private let events = EventListenerRegistry()
  private let vm: OnboardingLoginViewModel

  private let scrollView = UIScrollView()
  private let titleLabel = UILabel()
  private let bodyLabel = UILabel()
  private let field = UITextField()
  private let signInButton = BigButton(style: .regular)

  private var bottomConstraint: NSLayoutConstraint!

  weak var delegate: OnboardingViewControllerDelegate?

  init(viewModel: OnboardingLoginViewModel) {
    self.vm = viewModel
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func loadView() {
    super.loadView()
    self.view.backgroundColor = .systemBackground

    view.addSubview(scrollView)
    [titleLabel, bodyLabel, field, signInButton].forEach { scrollView.addSubview($0) }

    titleLabel.font = .preferredFont(forTextStyle: .title1)
    titleLabel.text = NSLocalizedString("What's your instance?", comment: "Screen: Onboarding Sign in: Title")
    titleLabel.lineBreakMode = .byWordWrapping
    titleLabel.numberOfLines = 0
    bodyLabel.font = .preferredFont(forTextStyle: .body)
    bodyLabel.text = NSLocalizedString("Please enter the URL you use to access your Chameleon instance.\n\nYou'll have the opportunity to sign in using your username and password at this address.", comment: "Screen: Onboarding Sign in: Body")
    bodyLabel.lineBreakMode = .byWordWrapping
    bodyLabel.numberOfLines = 0
    field.font = .preferredFont(forTextStyle: .body)
    field.placeholder = "https://chameleon.social"
    field.keyboardType = .URL
    field.autocorrectionType = .no
    field.autocapitalizationType = .none
    signInButton.setTitle(NSLocalizedString("Sign in", comment: "Screen: Onboarding Sign in: Sign in button"), for: .normal)

    constrain(scrollView, titleLabel, bodyLabel, field, signInButton) { container, title, body, field, signInButton in
      title.top == container.safeAreaLayoutGuide.top + 20
      title.leading == container.safeAreaLayoutGuide.leading + 20
      title.trailing == container.safeAreaLayoutGuide.trailing - 20

      body.top == title.bottom + 20
      body.leading == title.leading
      body.trailing == title.trailing

      field.top == body.bottom + 40
      field.leading == title.leading
      field.trailing == title.trailing

      signInButton.leading == title.leading
      signInButton.trailing == title.trailing
      signInButton.bottom == container.safeAreaLayoutGuide.bottom
    }

    constrain(view, scrollView) { container, scrollView in
      scrollView.top == container.top
      scrollView.leading == container.leading
      scrollView.trailing == container.trailing
      bottomConstraint = scrollView.bottom == container.bottom
    }
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    title = NSLocalizedString("Choose your instance", comment: "Screen: Title: Onboarding Server Select")
    NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillShow(sender:)), name: UIResponder.keyboardWillShowNotification, object: nil)
    NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillHide(sender:)), name: UIResponder.keyboardWillHideNotification, object: nil)
    events <~ vm.on("statechange") { [weak self] event in
      guard let current = event.data["current"] as? OnboardingLoginViewModelState else { return }
      let previous = event.data["previous"] as? OnboardingLoginViewModelState ?? current
      self?.handleTransition(from: previous, to: current)
    }
  }

  override func viewDidAppear(_ animated: Bool) {
    super.viewDidAppear(animated)
    vm.viewDidAppear()
    field.becomeFirstResponder()
  }

  private func handleTransition(from: OnboardingLoginViewModelState, to: OnboardingLoginViewModelState) {
    switch to.identifier {
    case .initial:
      break
    }
  }

  @objc private func keyboardWillShow(sender: Notification) {
    guard let userInfo = sender.userInfo else {
      return
    }

    guard let keyboardSizeValue = userInfo[UIResponder.keyboardFrameEndUserInfoKey] as? NSValue else {
      return
    }

    let keyboardScreenEndFrame = keyboardSizeValue.cgRectValue
    let keyboardViewEndFrame = view.convert(keyboardScreenEndFrame, from: view.window)

    bottomConstraint.constant = 0 - keyboardViewEndFrame.height - view.safeAreaInsets.bottom
    self.scrollView.setNeedsLayout()
    self.view.setNeedsLayout()
    UIView.animate(withDuration: 0.1) {
      self.view.layoutIfNeeded()
    }
  }

  @objc private func keyboardWillHide(sender: Notification) {
    bottomConstraint.constant = 0
    self.scrollView.setNeedsLayout()
    self.view.setNeedsLayout()
    UIView.animate(withDuration: 0.1) {
      self.view.layoutIfNeeded()
    }
  }
}
