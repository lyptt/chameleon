import { ButtonHTMLAttributes } from 'react'
import cx from 'classnames'

export default function Button({
  className,
  children,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button className={cx('chameleon-button', className)} {...props}>
      {children}
    </button>
  )
}
