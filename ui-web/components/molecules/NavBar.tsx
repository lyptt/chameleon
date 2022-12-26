import Config from '@/core/config'
import cx from 'classnames'
import Link from 'next/link'
import { HTMLAttributes } from 'react'
import UserButton from '@/components/molecules/UserButton'
import SearchBar from '../quarks/SearchBar'

export interface NavBarProps extends HTMLAttributes<HTMLDivElement> {
  isBuiltInRoute?: boolean
}

export default function NavBar({ className, isBuiltInRoute }: NavBarProps) {
  return (
    <nav className={cx('orbit-nav', className)}>
      <Link legacyBehavior href="/">
        <a className="orbit-nav__logo">
          <div className="orbit-nav__logo-top">
            <img
              className="orbit-nav__logo-image"
              alt="Orbit"
              src="/images/logo-light.svg"
              draggable="false"
            />
            <span className="orbit-nav__logo-text">orbit</span>
          </div>
          <span className="orbit-nav__logo-subtitle">
            {new URL(Config.fqdn!).hostname}
          </span>
        </a>
      </Link>
      {!isBuiltInRoute && <SearchBar className="orbit-nav__search-bar" />}
      <UserButton />
    </nav>
  )
}
