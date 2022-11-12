import cx from 'classnames'
import Link from 'next/link'
import { useRouter } from 'next/router'
import UserButton from '../atoms/UserButton'
import classNames from './Nav.module.css'
import {
  IoHomeOutline,
  IoHome,
  IoSearchOutline,
  IoSearch,
  IoCompassOutline,
  IoCompass,
  IoPaperPlaneOutline,
  IoPaperPlane,
  IoHeartOutline,
  IoHeart,
  IoAddCircleOutline,
} from 'react-icons/io5'

export interface INavProps {
  className?: string
}

interface INavItemProps {
  active?: boolean
  title: string
  href: string
  inactiveIcon: any
  activeIcon: any
}

function NavItem({
  active,
  title,
  href,
  inactiveIcon: InactiveIcon,
  activeIcon: ActiveIcon,
}: INavItemProps) {
  return (
    <li>
      <Link href={href}>
        <a className={classNames.link}>
          {active && <ActiveIcon />} {!active && <InactiveIcon />}{' '}
          <span>{title}</span>
        </a>
      </Link>
    </li>
  )
}

export default function Nav({ className }: INavProps) {
  const { route } = useRouter()

  return (
    <nav className={cx(className, classNames.container)}>
      <div className={cx(className, classNames.nav)}>
        <h1 className={classNames.title}>Chameleon</h1>
        <ul className={classNames.list} role="list">
          <NavItem
            active={route === '/'}
            title="Home"
            href="/"
            inactiveIcon={IoHomeOutline}
            activeIcon={IoHome}
          />
          <NavItem
            active={route === '/search'}
            title="Search"
            href="/search"
            inactiveIcon={IoSearchOutline}
            activeIcon={IoSearch}
          />
          <NavItem
            active={route === '/explore'}
            title="Explore"
            href="/explore"
            inactiveIcon={IoCompassOutline}
            activeIcon={IoCompass}
          />
          <NavItem
            active={route === '/messages'}
            title="Messages"
            href="/messages"
            inactiveIcon={IoPaperPlaneOutline}
            activeIcon={IoPaperPlane}
          />
          <NavItem
            active={route === '/notifications'}
            title="Notifications"
            href="/notifications"
            inactiveIcon={IoHeartOutline}
            activeIcon={IoHeart}
          />
          <li>
            <button className={classNames.link}>
              <IoAddCircleOutline />
              <span>Post!</span>
            </button>
          </li>
          <li>
            <UserButton active={route === '/profile'} />
          </li>
        </ul>
      </div>
    </nav>
  )
}
