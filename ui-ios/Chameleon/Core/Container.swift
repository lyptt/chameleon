import Foundation
import UIKit
import Swinject

class AppContainer {
  static let root: Container = {
    let c = Container()

    // MARK: View controllers

    c.register(SessionTransitionViewController.self) { r in
      SessionTransitionViewController(viewModel: r.resolve(SessionTransitionViewModel.self)!)
    }.inObjectScope(.transient)

    c.register(OnboardingViewController.self) { r in
      OnboardingViewController(viewModel: r.resolve(OnboardingViewModel.self)!)
    }.inObjectScope(.transient)

    c.register(OnboardingLoginViewController.self) { r in
      OnboardingLoginViewController(viewModel: r.resolve(OnboardingLoginViewModel.self)!)
    }.inObjectScope(.transient)

    c.register(HomeViewController.self) { r in
      HomeViewController(api: r.resolve(Api.self)!)
    }.inObjectScope(.transient)

    c.register(SearchViewController.self) { _ in
      SearchViewController()
    }.inObjectScope(.transient)

    c.register(MessagesViewController.self) { _ in
      MessagesViewController()
    }.inObjectScope(.transient)

    c.register(ProfileViewController.self) { _ in
      ProfileViewController()
    }.inObjectScope(.transient)

    c.register(MainViewController.self) { r in
      let home = UINavigationController(rootViewController: r.resolve(HomeViewController.self)!)
      let search = UINavigationController(rootViewController: r.resolve(SearchViewController.self)!)
      let messages = UINavigationController(rootViewController: r.resolve(MessagesViewController.self)!)
      let profile = UINavigationController(rootViewController: r.resolve(ProfileViewController.self)!)

      return MainViewController(children: [home, search, messages, profile])
    }.inObjectScope(.transient)

    // MARK: Services
    c.register(Api.self) { _ in
      DefaultApi()
    }.inObjectScope(.container)

    // MARK: View Models
    c.register(SessionTransitionViewModel.self) { r in
      SessionTransitionViewModel(api: r.resolve(Api.self)!)
    }.inObjectScope(.transient)

    c.register(OnboardingViewModel.self) { _ in
      OnboardingViewModel()
    }.inObjectScope(.transient)

    c.register(OnboardingLoginViewModel.self) { r in
      OnboardingLoginViewModel(api: r.resolve(Api.self)!)
    }.inObjectScope(.transient)

    return c
  }()
}
