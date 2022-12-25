import cx from 'classnames'
import { HTMLAttributes } from 'react'

export default function NavBar({ className }: HTMLAttributes<HTMLDivElement>) {
  return <nav className={cx('orbit-nav', className)}></nav>
}
