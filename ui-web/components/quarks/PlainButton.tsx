import { ButtonHTMLAttributes } from 'react'
import classNames from './PlainButton.module.css'
import cx from 'classnames'

export interface IPlainButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement> {
  brand?: boolean
  faded?: boolean
  thin?: boolean
}

export default function PlainButton({
  className,
  children,
  brand,
  faded,
  thin,
  ...props
}: IPlainButtonProps) {
  return (
    <button
      className={cx('chameleon-button--plain', classNames.button, className, {
        [classNames.brand]: brand,
        [classNames.faded]: faded,
        [classNames.thin]: thin,
      })}
      {...props}
    >
      {children}
    </button>
  )
}
