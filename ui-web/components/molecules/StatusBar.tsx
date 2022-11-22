import { HTMLAttributes } from 'react'
import cx from 'classnames'

export default function StatusBar({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cx('chameleon-status-bar', className)} {...props}>
      <div className="chameleon-status-bar__backdrop" aria-hidden="true" />
      <div className="chameleon-status-bar__content">{children}</div>
    </div>
  )
}
