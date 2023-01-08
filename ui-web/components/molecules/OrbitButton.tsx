import cx from 'classnames'
import { HTMLProps } from 'react'
import Link from 'next/link'
import { IOrbit } from '@/core/api'
import { cdnUrl } from '@/core/utils'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface OrbitButtonProps extends HTMLProps<HTMLAnchorElement> {
  specificProfile?: boolean
  orbit?: IOrbit
}

export default function OrbitButton({
  specificProfile,
  orbit,
  className,
  ...rest
}: OrbitButtonProps) {
  return (
    <Link legacyBehavior href={orbit ? `/orbits/${orbit.shortcode}` : '#'}>
      <a
        className={cx(
          'orbit-orbit-button',
          'orbit-orbit-button--authenticated',
          className
        )}
        {...rest}
      >
        {!!orbit && (
          <>
            <img
              className="orbit-orbit-button__avatar"
              src={cdnUrl(orbit.avatar_uri || transparentPixelUri)}
              alt={orbit.shortcode}
            />
            <div className="orbit-orbit-button__details">
              <div className="orbit-orbit-button__details-handle">
                {orbit.shortcode}
              </div>
              <div className="orbit-orbit-button__details-full-handle">
                {orbit.fediverse_id}
              </div>
            </div>
          </>
        )}
      </a>
    </Link>
  )
}
