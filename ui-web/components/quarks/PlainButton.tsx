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
  disabled,
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
          'chameleon-button--enabled': !disabled,
          'chameleon-button--disabled': disabled,
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
        'chameleon-plain-button--enabled': !disabled,
        'chameleon-plain-button--disabled': disabled,
      })}
      draggable={false}
      disabled={disabled}
      {...props}
    >
      {children}
    </button>
  )
}
