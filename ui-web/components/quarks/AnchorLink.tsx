import { HTMLProps } from 'react'
import cx from 'classnames'
import Link from 'next/link'

export default function AnchorLink({
  className,
  target,
  href,
  ...rest
}: HTMLProps<HTMLAnchorElement>) {
  if (target === 'blank') {
    return (
      <a
        className={cx('orbit-link', className)}
        target={target}
        href={href}
        {...rest}
      />
    )
  }

  return (
    <Link legacyBehavior href={href || ''}>
      <a className={cx('orbit-link', className)} {...rest} />
    </Link>
  )
}
