import { HTMLProps } from 'react'
import cx from 'classnames'

export default function SearchBar({
  className,
  ...props
}: HTMLProps<HTMLInputElement>) {
  return (
    <input
      className={cx('orbit-search-bar', className)}
      {...props}
      placeholder="Search"
    />
  )
}
