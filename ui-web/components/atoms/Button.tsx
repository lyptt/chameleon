import { ButtonHTMLAttributes, DetailedHTMLProps } from 'react'
import cx from 'classnames'

export interface ButtonProps
  extends DetailedHTMLProps<
    ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  href?: string
  variant?: 'default' | 'outline'
}

export default function Button({
  className,
  variant,
  href,
  ...rest
}: ButtonProps) {
  if (!!href) {
    return (
      <a
        href={href}
        className={cx(
          'orbit-button',
          { 'orbit-button--brand': !variant || variant === 'default' },
          { 'orbit-button--outline': variant === 'outline' },
          className
        )}
        {...(rest as any)}
      />
    )
  }

  return (
    <button
      className={cx(
        'orbit-button',
        { 'orbit-button--brand': !variant || variant === 'default' },
        { 'orbit-button--outline': variant === 'outline' },
        className
      )}
      {...rest}
    />
  )
}
