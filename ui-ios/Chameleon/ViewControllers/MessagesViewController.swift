import UIKit

class MessagesViewController: UIViewController {
  init() {
    super.init(nibName: nil, bundle: nil)
    tabBarItem = UITabBarItem(title: NSLocalizedString("Messages", comment: "Tab: Item: Messages"), image: UIImage(systemName: "tray.full"), tag: 2)
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
    title = NSLocalizedString("Messages", comment: "Screen: Title: Messages")
  }
}
