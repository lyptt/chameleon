import UIKit

class SearchViewController: UIViewController {
  init() {
    super.init(nibName: nil, bundle: nil)
    tabBarItem = UITabBarItem(title: NSLocalizedString("Search", comment: "Tab: Item: Search"), image: UIImage(systemName: "magnifyingglass"), tag: 1)
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
    title = NSLocalizedString("Search", comment: "Screen: Title: Search")
  }
}
