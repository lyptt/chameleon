import { ButtonHTMLAttributes } from 'react'
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
      className={cx('chameleon-button--plain', className, {
        'chameleon-button--plain-brand': brand,
        'chameleon-button--plain-faded': faded,
        'chameleon-button--plain-thin': thin,
      })}
      {...props}
    >
      {children}
    </button>
  )
}
