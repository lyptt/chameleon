import { HTMLAttributes } from 'react'
import cx from 'classnames'
import { IoSearch } from 'react-icons/io5'

export default function StatusBar({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return <div className={cx('chameleon-status-bar', className)} {...props} />
}
