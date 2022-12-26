import UIKit

protocol MainViewControllerTab {
  var session: Session? { get set }
}

class MainViewController: UITabBarController, MainViewControllerTab {
  var session: Session? {
    didSet {
      (viewControllers ?? []).forEach {
        guard let nvc = $0 as? UINavigationController else {
          return
        }

        guard var vc = nvc.viewControllers.first as? MainViewControllerTab else {
          return
        }

        vc.session = session
      }
    }
  }

  init(children: [UINavigationController]) {
    super.init(nibName: nil, bundle: nil)
    viewControllers = children
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func viewDidLoad() {
    super.viewDidLoad()
  }
}
