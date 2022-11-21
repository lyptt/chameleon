import { ButtonHTMLAttributes } from 'react'
import cx from 'classnames'

export interface IPlainButtonProps extends ButtonHTMLAttributes<any> {
  brand?: boolean
  faded?: boolean
  thin?: boolean
  href?: string
}

export default function PlainButton({
  className,
  children,
  brand,
  faded,
  thin,
  href,
  ...props
}: IPlainButtonProps) {
  if (!!href) {
    return (
      <a
        href={href}
        className={cx('chameleon-plain-button', className, {
          'chameleon-plain-button--brand': brand,
          'chameleon-plain-button--faded': faded,
          'chameleon-plain-button--thin': thin,
        })}
        draggable={false}
        {...props}
      >
        {children}
      </a>
    )
  }

  return (
    <button
      className={cx('chameleon-plain-button', className, {
        'chameleon-plain-button--brand': brand,
        'chameleon-plain-button--faded': faded,
        'chameleon-plain-button--thin': thin,
      })}
      draggable={false}
      {...props}
    >
      {children}
    </button>
  )
}
