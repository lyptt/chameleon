import Foundation
import UIKit
import Cartography

class TitleView: UIView {
  private static let padding: CGFloat = 8
  private static let paddingBottom: CGFloat = 2
  let imageView = UIImageView()

  override init(frame: CGRect) {
    super.init(frame: frame)

    let image = UIImage(named: "FullLogo")!
    imageView.image = image
    imageView.contentMode = .scaleAspectFit

    addSubview(imageView)
  }

  required init?(coder: NSCoder) {
    fatalError()
  }

  override func layoutSubviews() {
    super.layoutSubviews()

    imageView.sizeToFit()

    let size = imageView.bounds.size
    let targetSize = CGSize(width: bounds.width - (TitleView.padding * 2), height: bounds.height - TitleView.padding)

    let widthRatio  = targetSize.width  / size.width
    let heightRatio = targetSize.height / size.height

    var newSize: CGSize
    if(widthRatio > heightRatio) {
      newSize = CGSize(width: size.width * heightRatio, height: size.height * heightRatio)
    } else {
      newSize = CGSize(width: size.width * widthRatio, height: size.height * widthRatio)
    }

    imageView.frame = CGRect(x: TitleView.padding, y: TitleView.padding, width: newSize.width, height: newSize.height)
  }
}
