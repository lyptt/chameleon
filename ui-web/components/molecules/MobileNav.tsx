import { HTMLAttributes, useState } from 'react'
import cx from 'classnames'
import { INewPost, AccessType } from '@/core/api'
import Link from 'next/link'
import { useRouter } from 'next/router'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  useFeed,
  feedActionSubmitPost,
} from '@/components/organisms/FeedContext'
import NewPostModal from './NewPostModal'
import Button from '@/components/quarks/Button'
import PlainButton from '@/components/quarks/PlainButton'

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
      <Link href={href} className="chameleon-mobile-nav__link" title={title}>
        {active && <ActiveIcon />} {!active && <InactiveIcon />}{' '}
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
      <nav className={cx('chameleon-mobile-nav', className)}>
        <Link className="chameleon-mobile-nav__title-link" href="/">
          <h1 className="chameleon-mobile-nav__title">Chameleon</h1>
        </Link>
        {!session && (
          <>
            <Button
              href="/api/oauth/authorize"
              className="chameleon-mobile-nav__login-button"
              bold
            >
              Sign in
            </Button>
            <PlainButton
              href="/api/oauth/authorize"
              className="chameleon-mobile__register-button"
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
