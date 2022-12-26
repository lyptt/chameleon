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

export default function Button({ className, variant, ...rest }: ButtonProps) {
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
