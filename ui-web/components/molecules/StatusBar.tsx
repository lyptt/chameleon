import { HTMLAttributes } from 'react'
import cx from 'classnames'
import Link from 'next/link'

export interface IStatusBarProps extends HTMLAttributes<HTMLDivElement> {
  href?: string
}

export default function StatusBar({
  className,
  children,
  href,
  ...props
}: IStatusBarProps) {
  return (
    <div className={cx('chameleon-status-bar', className)} {...props}>
      <div className="chameleon-status-bar__backdrop" aria-hidden="true" />
      {!!href && (
        <Link href={href} className="chameleon-status-bar__content">
          {children}
        </Link>
      )}
      {!href && <div className="chameleon-status-bar__content">{children}</div>}
    </div>
  )
}
