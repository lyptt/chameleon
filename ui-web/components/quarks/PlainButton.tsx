import { ButtonHTMLAttributes } from 'react'
import classNames from './PlainButton.module.css'
import cx from 'classnames'

export interface IPlainButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement> {
  brand?: boolean
}

export default function PlainButton({
  className,
  children,
  brand,
  ...props
}: IPlainButtonProps) {
  return (
    <button
      className={cx('chameleon-button--plain', classNames.button, className, {
        [classNames.brand]: brand,
      })}
      {...props}
    >
      {children}
    </button>
  )
}
