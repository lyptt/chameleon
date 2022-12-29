import { HTMLProps } from 'react'
import cx from 'classnames'
import Button from './Button'

export interface InfoCardAction {
  title: string
  href: string
  button: 'default' | 'outline'
}

export interface InfoCardProps extends HTMLProps<HTMLDivElement> {
  title: string
  innerHTML?: string
  actions?: InfoCardAction[]
}

export default function InfoCard({
  title,
  innerHTML,
  actions,
  children,
  className,
  ...rest
}: InfoCardProps) {
  return (
    <div className={cx('orbit-info-card', className)} {...rest}>
      <div className="orbit-info-card__header">{title}</div>
      {!!innerHTML && (
        <div
          className="orbit-info-card__content"
          dangerouslySetInnerHTML={{ __html: innerHTML }}
        />
      )}
      {!innerHTML && <div className="orbit-info-card__content">{children}</div>}
      {!!innerHTML && children && (
        <div className="orbit-info-card__additional-content">{children}</div>
      )}
      {!!actions && (
        <>
          <div role="separator" className="orbit-info-card__separator" />
          <div className="orbit-info-card__actions">
            {actions.map((action) => (
              <Button key={action.title + action.href} href={action.href}>
                {action.title}
              </Button>
            ))}
          </div>
        </>
      )}
    </div>
  )
}
