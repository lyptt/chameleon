import { HTMLAttributes, useState } from 'react'
import classNames from './MobileNav.module.css'
import cx from 'classnames'
import { INewPost, AccessType } from '@/core/api'
import Link from 'next/link'
import { useRouter } from 'next/router'
import {
  IoHomeOutline,
  IoHome,
  IoCompass,
  IoCompassOutline,
  IoHeart,
  IoHeartOutline,
  IoPaperPlane,
  IoPaperPlaneOutline,
  IoSearch,
  IoSearchOutline,
  IoAddCircleOutline,
} from 'react-icons/io5'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  useFeed,
  feedActionSubmitPost,
} from '@/components/organisms/FeedContext'
import NewPostModal from './NewPostModal'
import UserButton from '../atoms/UserButton'

interface INavItemProps {
  active?: boolean
  title: string
  href: string
  inactiveIcon: any
  activeIcon: any
}

function MobileNavItem({
  active,
  title,
  href,
  inactiveIcon: InactiveIcon,
  activeIcon: ActiveIcon,
}: INavItemProps) {
  return (
    <li>
      <Link href={href}>
        <a
          className={cx('chameleon-mobile-nav__link', classNames.link)}
          title={title}
        >
          {active && <ActiveIcon />} {!active && <InactiveIcon />}{' '}
        </a>
      </Link>
    </li>
  )
}

export default function MobileNav({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { route } = useRouter()
  const { session } = useAuth()
  const { dispatch } = useFeed()
  const [newPostModalOpen, setNewPostModalOpen] = useState(false)

  const handleModalOpen = () => setNewPostModalOpen(true)
  const handleModalClose = () => setNewPostModalOpen(false)
  const handleModalSubmit = (
    visibility: string,
    file: File,
    contentMd: string
  ) => {
    setNewPostModalOpen(false)
    const newPost: INewPost = {
      content_md: contentMd,
      visibility: visibility as AccessType,
    }

    feedActionSubmitPost(newPost, file, session?.access_token, dispatch)
  }

  return (
    <>
      <nav className={cx('chameleon-mobile-nav', classNames.nav, className)}>
        <ul
          className={cx('chameleon-mobile-nav__list', classNames.list)}
          role="list"
        >
          <MobileNavItem
            active={route === '/'}
            title="Home"
            href="/"
            inactiveIcon={IoHomeOutline}
            activeIcon={IoHome}
          />
          <MobileNavItem
            active={route === '/search'}
            title="Search"
            href="/search"
            inactiveIcon={IoSearchOutline}
            activeIcon={IoSearch}
          />
          <MobileNavItem
            active={route === '/explore'}
            title="Explore"
            href="/explore"
            inactiveIcon={IoCompassOutline}
            activeIcon={IoCompass}
          />
          <MobileNavItem
            active={route === '/messages'}
            title="Messages"
            href="/messages"
            inactiveIcon={IoPaperPlaneOutline}
            activeIcon={IoPaperPlane}
          />
          <MobileNavItem
            active={route === '/notifications'}
            title="Notifications"
            href="/notifications"
            inactiveIcon={IoHeartOutline}
            activeIcon={IoHeart}
          />
          <li>
            <button className={classNames.link} onClick={handleModalOpen}>
              <IoAddCircleOutline />
            </button>
          </li>
          <li>
            <UserButton active={route === '/profile'} />
          </li>
        </ul>
      </nav>

      <NewPostModal
        open={newPostModalOpen}
        onCancel={handleModalClose}
        onSubmit={handleModalSubmit}
      />
    </>
  )
}
