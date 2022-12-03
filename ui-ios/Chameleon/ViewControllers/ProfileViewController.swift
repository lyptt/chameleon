import UIKit

class ProfileViewController: UIViewController {
  init() {
    super.init(nibName: nil, bundle: nil)
    tabBarItem = UITabBarItem(title: NSLocalizedString("Profile", comment: "Tab: Item: Profile"), image: UIImage(systemName: "person.crop.circle"), tag: 3)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func loadView() {
    super.loadView()
    self.view.backgroundColor = .systemBackground
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    title = NSLocalizedString("Profile", comment: "Screen: Title: Profile")
  }
}
