import Foundation
import UIKit

enum BigButtonStyle {
  case regular
  case alternate
}

class BigButton: UIButton {
  private var style: BigButtonStyle = .regular

  convenience init(style: BigButtonStyle) {
    self.init(type: .custom)
    self.style = style
    setup()
  }

  private func setup() {
    setTitleColor(.white, for: .normal)
    titleLabel?.font = .preferredFont(forTextStyle: .callout)
    backgroundColor = determineBackgroundColor(for: .normal)
    layer.cornerRadius = 5
  }

  override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
    super.touchesBegan(touches, with: event)
    UIView.animate(withDuration: 0.1, delay: 0, options: [.curveEaseOut]) {
      self.backgroundColor = self.determineBackgroundColor(for: .highlighted)
    }
  }

  override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
    super.touchesEnded(touches, with: event)
    UIView.animate(withDuration: 0.1, delay: 0, options: [.curveEaseOut]) {
      self.backgroundColor = self.determineBackgroundColor(for: .normal)
    }
  }

  override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
    super.touchesCancelled(touches, with: event)
    UIView.animate(withDuration: 0.1, delay: 0, options: [.curveEaseOut]) {
      self.backgroundColor = self.determineBackgroundColor(for: .normal)
    }
  }

  override var intrinsicContentSize: CGSize {
    var size = super.intrinsicContentSize
    size.height += 6
    return size
  }

  private func determineBackgroundColor(for state: UIControl.State) -> UIColor {
    switch style {
    case .regular:
      return state == .focused || state == .highlighted ? .tintColor.withLuminosity(0.2) : .tintColor
    case .alternate:
      return state == .focused || state == .highlighted ? .alternateTintColor.withLuminosity(0.2) : .alternateTintColor
    }
  }
}
