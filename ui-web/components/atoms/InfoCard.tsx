import { HTMLProps, MouseEventHandler } from 'react'
import cx from 'classnames'
import Button from './Button'
import { IOrbit, IProfile } from '@/core/api'
import UserButton from '../molecules/UserButton'
import { cdnUrl } from '@/core/utils'
import OrbitButton from '../molecules/OrbitButton'

export interface InfoCardAction {
  title: string
  href?: string
  action?: MouseEventHandler<HTMLButtonElement>
  button: 'default' | 'outline'
}

export interface InfoCardProps extends HTMLProps<HTMLDivElement> {
  title?: string
  titleImageUrl?: string
  author?: IProfile
  orbit?: IOrbit
  innerHTML?: string
  actions?: InfoCardAction[]
  slim?: boolean
}

export default function InfoCard({
  title,
  titleImageUrl,
  author,
  orbit,
  innerHTML,
  actions,
  children,
  className,
  slim,
  ...rest
}: InfoCardProps) {
  return (
    <div className={cx('orbit-info-card', className)} {...rest}>
      {!author && !orbit && (
        <div className="orbit-info-card__header">
          {title}{' '}
          {!!titleImageUrl && (
            <img
              className="orbit-info-card__header-image"
              src={cdnUrl(titleImageUrl)}
              alt={title}
              draggable={false}
            />
          )}
        </div>
      )}
      {!!author && (
        <div className="orbit-info-card__header">
          <UserButton specificProfile profile={author} />
        </div>
      )}
      {!!orbit && (
        <div className="orbit-info-card__header">
          <OrbitButton specificProfile orbit={orbit} />
        </div>
      )}
      {!!innerHTML && (
        <div
          className="orbit-info-card__content"
          dangerouslySetInnerHTML={{ __html: innerHTML }}
        />
      )}
      {!slim && !innerHTML && (
        <div className="orbit-info-card__content">{children}</div>
      )}
      {!slim && !!innerHTML && children && (
        <div className="orbit-info-card__additional-content">{children}</div>
      )}
      {!!actions && (
        <>
          <div role="separator" className="orbit-info-card__separator" />
          <div className="orbit-info-card__actions">
            {actions.map((action) => (
              <Button
                key={action.title}
                href={action.href}
                onClick={action.action}
                variant={action.button}
              >
                {action.title}
              </Button>
            ))}
          </div>
        </>
      )}
    </div>
  )
}
