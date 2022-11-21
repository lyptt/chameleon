import cx from 'classnames'
import Link from 'next/link'
import { useRouter } from 'next/router'
import {
  IoAperture,
  IoEarth,
  IoHome,
  IoNotifications,
  IoPeople,
} from 'react-icons/io5'
import NewPostModal from './NewPostModal'
import { useState } from 'react'
import {
  feedActionSubmitPost,
  useFeed,
} from '@/components/organisms/FeedContext'
import { useAuth } from '@/components/organisms/AuthContext'
import { AccessType, INewPost } from '@/core/api'
import Button from '@/components/quarks/Button'
import PlainButton from '@/components/quarks/PlainButton'

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
    <li className="chameleon-nav__item">
      <Link
        href={href}
        className={cx('chameleon-nav__link', {
          'chameleon-nav__link--active': active,
        })}
      >
        {active && <ActiveIcon className="chameleon-nav__link-icon" />}{' '}
        {!active && <InactiveIcon className="chameleon-nav__link-icon" />}{' '}
        <span className="chameleon-nav__link-text">{title}</span>
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
      <nav className={cx('chameleon-nav', className)}>
        <Link className="chameleon-nav__title-link" href="/">
          <h1 className="chameleon-nav__title">Chameleon</h1>
        </Link>
        <hr
          className="chameleon-nav__separator chameleon-nav__link-separator"
          aria-hidden="true"
        />
        <ul className="chameleon-nav__list" role="list">
          <NavItem
            active={route === '/'}
            title={session ? 'Home' : 'Explore'}
            href="/"
            inactiveIcon={session ? IoHome : IoAperture}
            activeIcon={session ? IoHome : IoAperture}
          />
          {!session && (
            <>
              <NavItem
                active={route === '/public/local'}
                title="Local"
                href="/public/local"
                inactiveIcon={IoPeople}
                activeIcon={IoPeople}
              />
              <NavItem
                active={route === '/public/federated'}
                title="Federated"
                href="/public/federated"
                inactiveIcon={IoEarth}
                activeIcon={IoEarth}
              />
            </>
          )}
          {!!session && (
            <>
              <NavItem
                active={route === '/notifications'}
                title="Notifications"
                href="/notifications"
                inactiveIcon={IoNotifications}
                activeIcon={IoNotifications}
              />
            </>
          )}
        </ul>
        <hr className="chameleon-nav__separator" aria-hidden="true" />
        {!session && (
          <>
            <p className="chameleon-nav__login-cta">
              Sign in to follow profiles or hashtags, favorite, share and reply
              to posts, or interact from your account on a different server.
            </p>
            <Button
              href="/api/oauth/authorize"
              className="chameleon-nav__login-button"
              bold
            >
              Sign in
            </Button>
            <PlainButton
              href="/api/oauth/authorize"
              className="chameleon-nav__register-button"
              brand
            >
              Create account
            </PlainButton>
          </>
        )}
      </nav>
      <NewPostModal
        open={newPostModalOpen}
        onCancel={handleModalClose}
        onSubmit={handleModalSubmit}
      />
    </>
  )
}
