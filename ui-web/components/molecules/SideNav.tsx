import { cdnUrl } from '@/core/utils'
import cx from 'classnames'
import Link from 'next/link'
import { HTMLAttributes } from 'react'
import { IoEarthOutline, IoHomeOutline, IoPersonOutline } from 'react-icons/io5'
import { useAuth } from '../organisms/AuthContext'
import { useOrbits } from '../organisms/OrbitContext'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export default function SideNav({ className }: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state } = useOrbits()
  const { orbits } = state

  return (
    <aside className={cx('orbit-side-nav', className)}>
      <div className="orbit-side-nav__inner">
        <div className="orbit-side-nav__header">Feeds</div>
        <ul className="orbit-side-nav__list">
          <li className="orbit-side-nav__list-item">
            <Link legacyBehavior href="/">
              <a>
                <IoHomeOutline className="orbit-side-nav__list-item-icon" />
                Home
              </a>
            </Link>
          </li>

          {!!session && (
            <li className="orbit-side-nav__list-item">
              <Link legacyBehavior href="/feed/friends">
                <a>
                  <IoPersonOutline className="orbit-side-nav__list-item-icon" />
                  Friends
                </a>
              </Link>
            </li>
          )}

          <li className="orbit-side-nav__list-item">
            <Link legacyBehavior href="/feed/federated">
              <a>
                <IoEarthOutline className="orbit-side-nav__list-item-icon" />
                Fediverse
              </a>
            </Link>
          </li>
        </ul>

        {!!session && <div className="orbit-side-nav__header">Orbits</div>}
        {!session && (
          <div className="orbit-side-nav__header">Popular Orbits</div>
        )}
        <ul className="orbit-side-nav__list">
          {!!orbits &&
            orbits.map((orbit) => (
              <li key={orbit.orbit_id} className="orbit-side-nav__list-item">
                <Link legacyBehavior href={`/orbits/${orbit.shortcode}`}>
                  <a>
                    <img
                      className="orbit-side-nav__list-item-icon orbit-side-nav__list-item-icon--external"
                      src={cdnUrl(orbit.avatar_uri || transparentPixelUri)}
                      alt={orbit.name}
                    />
                    {orbit.name}
                  </a>
                </Link>
              </li>
            ))}
        </ul>
      </div>
    </aside>
  )
}
