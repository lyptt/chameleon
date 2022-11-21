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
  ...props
}: IButtonProps) {
  if (!!href) {
    return (
      <a
        href={href}
        className={cx(
          'chameleon-button',
          { 'chameleon-button--bold': bold },
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
        className
      )}
      draggable={false}
      {...props}
    >
      {children}
    </button>
  )
}
