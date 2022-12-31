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
  ...rest
}: ButtonProps) {
  if (!!href) {
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
