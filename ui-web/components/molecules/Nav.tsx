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
import NewPostModal from './NewPostModal'
import { useState } from 'react'
import { feedActionSubmitPost, useFeed } from '../organisms/FeedContext'
import { useAuth } from '../organisms/AuthContext'
import { AccessType, INewPost } from '@/core/api'

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
              <button className={classNames.link} onClick={handleModalOpen}>
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
      <NewPostModal
        open={newPostModalOpen}
        onCancel={handleModalClose}
        onSubmit={handleModalSubmit}
      />
    </>
  )
}
