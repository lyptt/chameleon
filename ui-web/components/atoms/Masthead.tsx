import { HTMLProps } from 'react'
import cx from 'classnames'
import { IOrbit } from '@/core/api'
import { cdnUrl } from '@/core/utils'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mMs8tnyHwAFHQJz8PWweQAAAABJRU5ErkJggg==`

export interface MastheadProps extends HTMLProps<HTMLDivElement> {
  orbit: IOrbit
}

export default function Masthead({ orbit, className, ...rest }: MastheadProps) {
  return (
    <div className={cx('orbit-masthead', className)} {...rest}>
      <img
        src={cdnUrl(orbit.banner_uri || transparentPixelUri)}
        alt={orbit.name}
        className="orbit-masthead__banner"
        draggable="false"
      />

      <div className="orbit-masthead__content">
        <div className="orbit-masthead__content-title">o/{orbit.shortcode}</div>
      </div>
    </div>
  )
}
