import { ButtonHTMLAttributes, DetailedHTMLProps } from 'react'
import cx from 'classnames'
import Link from 'next/link'

export interface ButtonProps
  extends DetailedHTMLProps<
    ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  href?: string
  target?: string
  variant?: 'default' | 'outline'
}

export default function Button({
  className,
  variant,
  href,
  target,
  disabled,
  ...rest
}: ButtonProps) {
  if (!!href && !disabled) {
    if (target === 'blank') {
      return (
        <a
          href={href}
          className={cx(
            'orbit-button',
            { 'orbit-button--brand': !variant || variant === 'default' },
            { 'orbit-button--outline': variant === 'outline' },
            className
          )}
          disabled={disabled}
          {...(rest as any)}
        />
      )
    }

    return (
      <Link
        href={href}
        className={cx(
          'orbit-button',
          { 'orbit-button--brand': !variant || variant === 'default' },
          { 'orbit-button--outline': variant === 'outline' },
          className
        )}
        disabled={disabled}
        {...(rest as any)}
      />
    )
  }

  return (
    <button
      type="button"
      className={cx(
        'orbit-button',
        { 'orbit-button--brand': !variant || variant === 'default' },
        { 'orbit-button--outline': variant === 'outline' },
        className
      )}
      disabled={disabled}
      {...rest}
    />
  )
}
