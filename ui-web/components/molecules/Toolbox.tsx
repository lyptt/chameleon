import { HTMLProps, useState } from 'react'
import cx from 'classnames'
import SearchField from '@/components/atoms/SearchField'
import { useAuth } from '@/components/organisms/AuthContext'
import Config from '@/core/config'
import Button from '@/components/quarks/Button'
import Link from 'next/link'
import UserProfileCard from '@/components/atoms/UserProfileCard'
import NewPostForm from '@/components/molecules/NewPostForm'
import { useRouter } from 'next/router'
import {
  feedActionSubmitPost,
  useFeed,
} from '@/components/organisms/FeedContext'
import { AccessType, INewPost } from '@/core/api'

export default function Toolbox({
  className,
  ...props
}: HTMLProps<HTMLDivElement>) {
  const { session } = useAuth()
  const { dispatch } = useFeed()
  const handleModalSubmit = (
    visibility: string,
    file: File,
    contentMd: string
  ) => {
    const newPost: INewPost = {
      content_md: contentMd,
      visibility: visibility as AccessType,
    }

    feedActionSubmitPost(newPost, file, session?.access_token, dispatch)
  }

  const fqdnUrl = new URL(Config.fqdn || 'about:blank')
  const fqdnSimplified = fqdnUrl.hostname

  return (
    <aside className={cx('chameleon-toolbox', className)} {...props}>
      <SearchField
        className="chameleon-toolbox__search-field"
        title={session ? 'Search or paste URL' : 'Search'}
      />
      {!session && (
        <>
          <p className="chameleon-toolbox__server-info-block">
            <span className="chameleon-toolbox__server-info-block--bold">
              {fqdnSimplified}
            </span>{' '}
            is part of the decentralized social network powered by{' '}
            <a
              className="chameleon-toolbox__server-info-block-link"
              href="https://github.com/lyptt/chameleon"
              target="blank"
              rel="noreferrer noopener"
            >
              Chameleon
            </a>
            .
          </p>
          <p className="chameleon-toolbox__server-info-block">
            The original dev server operated by volunteers contributing to the
            Chameleon project.
          </p>
          <Button
            href="https://github.com/lyptt/chameleon"
            target="blank"
            rel="noreferrer noopener"
          >
            Learn more
          </Button>
        </>
      )}

      {!!session && (
        <>
          <UserProfileCard className="chameleon-toolbox__user-profile" />
          <NewPostForm
            className="chameleon-toolbox__new-post-form"
            onSubmit={handleModalSubmit}
          />
        </>
      )}

      <span
        className="chameleon-toolbox__server-info-spacer"
        aria-hidden="true"
      />
      <p className="chameleon-toolbox__server-info-block chameleon-toolbox__server-info-block--trailing">
        {fqdnSimplified}{' '}
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="/about"
        >
          About
        </Link>
        ·
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="/users"
        >
          Profiles directory
        </Link>
        ·
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="/policies/privacy-policy"
        >
          Privacy policy
        </Link>
      </p>
      <p className="chameleon-toolbox__server-info-block chameleon-toolbox__server-info-block--trailing">
        <span className="chameleon-toolbox__server-info-block--bold">
          Chameleon:
        </span>
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="/apps/mobile"
        >
          Get the app
        </Link>
        ·
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="/help/keyboard-shortcuts"
        >
          Keyboard shortcuts
        </Link>
        ·
        <Link
          className="chameleon-toolbox__server-info-block-link chameleon-toolbox__server-info-block--trailing-link"
          href="https://github.com/lyptt/chameleon"
          target="blank"
          rel="noreferrer noopener"
        >
          View source code
        </Link>
      </p>
    </aside>
  )
}
