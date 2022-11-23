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
import {
  postActionAddPostComment,
  postActionDeselect,
  usePost,
} from '../organisms/PostContext'
import NewCommentForm from './NewCommentForm'

export default function Toolbox({
  className,
  ...props
}: HTMLProps<HTMLDivElement>) {
  const { session } = useAuth()
  const { dispatch: feedDispatch } = useFeed()
  const { state: postState, dispatch: postDispatch } = usePost()
  const { route } = useRouter()
  const handlePostSubmit = (
    visibility: string,
    file: File,
    contentMd: string
  ) => {
    const newPost: INewPost = {
      content_md: contentMd,
      visibility: visibility as AccessType,
    }

    feedActionSubmitPost(newPost, file, session?.access_token, feedDispatch)
  }

  const handleCommentSubmit = (comment: string, _visibility: string) => {
    const postId = postState.post?.post_id
    if (!postId) {
      return postActionDeselect(postDispatch)
    }

    postActionAddPostComment(
      comment,
      postId,
      session?.access_token,
      postDispatch
    )
  }

  const handleCommentCancel = () => postActionDeselect(postDispatch)

  const fqdnUrl = new URL(Config.fqdn || 'about:blank')
  const fqdnSimplified = fqdnUrl.hostname

  return (
    <aside className={cx('chameleon-toolbox', className)} {...props}>
      <div className="chameleon-toolbox__content">
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
            {route === '/' && (
              <NewPostForm
                className="chameleon-toolbox__new-post-form"
                onSubmit={handlePostSubmit}
              />
            )}
            {route === '/users/[userId]/[postId]' && (
              <NewCommentForm
                className="chameleon-toolbox__new-comment-form"
                onSubmit={handleCommentSubmit}
                onCancel={handleCommentCancel}
                comment={postState.selectedComment}
                post={postState.selectedPost}
              />
            )}
          </>
        )}
      </div>
      <div className="chameleon-toolbox__footer">
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
      </div>
    </aside>
  )
}
