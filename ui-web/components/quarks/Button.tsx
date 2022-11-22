import { ButtonHTMLAttributes } from 'react'
import cx from 'classnames'

export interface IButtonProps extends ButtonHTMLAttributes<any> {
  bold?: boolean
  href?: string
  target?: string
  rel?: string
}

export default function Button({
  className,
  children,
  bold,
  href,
  disabled,
  ...props
}: IButtonProps) {
  if (!!href) {
    return (
      <a
        href={href}
        className={cx(
          'chameleon-button',
          { 'chameleon-button--bold': bold },
          { 'chameleon-button--enabled': !disabled },
          { 'chameleon-button--disabled': disabled },
          className
        )}
        draggable={false}
        {...props}
      >
        {children}
      </a>
    )
  }

  return (
    <button
      className={cx(
        'chameleon-button',
        { 'chameleon-button--bold': bold },
        { 'chameleon-button--enabled': !disabled },
        { 'chameleon-button--disabled': disabled },
        className
      )}
      draggable={false}
      disabled={disabled}
      {...props}
    >
      {children}
    </button>
  )
}
