import { ButtonHTMLAttributes } from 'react'
import classNames from './Button.module.css'
import cx from 'classnames'

export default function Button({
  className,
  children,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      className={cx('chameleon-button', classNames.button, className)}
      {...props}
    >
      {children}
    </button>
  )
}
