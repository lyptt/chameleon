import UIKit

class HomeViewController: UITableViewController {
  private let api: Api

  init(api: Api) {
    self.api = api
    super.init(nibName: nil, bundle: nil)
    tabBarItem = UITabBarItem(title: NSLocalizedString("Home", comment: "Tab: Item: Home"), image: UIImage(systemName: "house"), tag: 0)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func loadView() {
    super.loadView()
    view.backgroundColor = .systemBackground
    tableView.separatorStyle = .none
    tableView.register(PostViewCell.self, forCellReuseIdentifier: "Post")
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    title = NSLocalizedString("Home", comment: "Screen: Title: Home")
    let titleView = TitleView(frame: CGRect(x: 0, y: 0, width: 9001, height: 42))
    navigationItem.titleView = titleView
    navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: .add, target: nil, action: nil)
  }
}
