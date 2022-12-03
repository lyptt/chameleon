import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
  var window: UIWindow?

  func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
    window = UIWindow(frame: UIScreen.main.bounds)

    let vc = AppContainer.root.resolve(SessionTransitionViewController.self)!
    vc.transition(to: .initialize)

    window!.rootViewController = UINavigationController(rootViewController: vc)
    window!.makeKeyAndVisible()
    return true
  }
}
