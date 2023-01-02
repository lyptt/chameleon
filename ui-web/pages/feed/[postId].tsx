import Head from 'next/head'
import react, { useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import SideNav from '@/components/molecules/SideNav'
import { useRouter } from 'next/router'
import InfoCard from '@/components/atoms/InfoCard'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsLocalizedFormat from 'dayjs/plugin/localizedFormat'
import {
  postActionDeleteComment,
  postActionLoadAuthor,
  postActionLoadComments,
  postActionLoadOrbit,
  postActionLoadPost,
  usePost,
} from '@/components/organisms/PostContext'
import PostContent from '@/components/molecules/PostContent'
import NewCommentModal from '@/components/molecules/NewCommentModal'
import { useProfile } from '@/components/organisms/ProfileContext'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsLocalizedFormat)

export default function PostPage({
  className,
}: react.HTMLAttributes<HTMLDivElement>) {
  const router = useRouter()
  const { session } = useAuth()
  const { state, dispatch } = usePost()
  const {
    state: { profile },
  } = useProfile()
  const {
    loading,
    loadingFailed,
    post,
    orbitLoading,
    orbitLoadingFailed,
    orbit,
    authorLoading,
    authorLoadingFailed,
    author,
    page,
    noMorePages,
    totalComments,
    comments,
    commentsLoading,
    commentsLoadingFailed,
    initialCommentLoadComplete,
  } = state
  const [commentModalOpen, setCommentModalOpen] = useState(false)

  const postId = (router.query.postId || '') as string

  useEffect(() => {
    if (!loading && !loadingFailed && !post) {
      postActionLoadPost(postId, session?.access_token, dispatch)
    }
  }, [post, postId, dispatch, session, loading, loadingFailed])

  useEffect(() => {
    if (
      !orbitLoading &&
      !orbitLoadingFailed &&
      !!post?.orbit_shortcode &&
      !orbit
    ) {
      postActionLoadOrbit(post.orbit_shortcode, session?.access_token, dispatch)
    }
  }, [orbit, post, dispatch, orbitLoading, orbitLoadingFailed, session])

  useEffect(() => {
    if (
      !authorLoading &&
      !authorLoadingFailed &&
      !!post &&
      !post.orbit_shortcode &&
      !author
    ) {
      postActionLoadAuthor(post.user_handle, session?.access_token, dispatch)
    }
  }, [author, post, dispatch, authorLoading, authorLoadingFailed, session])

  useEffect(() => {
    if (
      !initialCommentLoadComplete &&
      !!post &&
      !commentsLoading &&
      !commentsLoadingFailed
    ) {
      postActionLoadComments(post.post_id, 0, session?.access_token, dispatch)
    }
  }, [
    initialCommentLoadComplete,
    post,
    dispatch,
    commentsLoading,
    commentsLoadingFailed,
    session,
  ])

  const shouldDisplay =
    !loading &&
    !loadingFailed &&
    post &&
    post.post_id === postId &&
    post.orbit_id == orbit?.orbit_id

  let title = 'Orbit'
  if (post) {
    if (post.title) {
      if (post.orbit_shortcode) {
        title = `${title} - u/${post.orbit_shortcode} - ${post.title}`
      } else {
        title = `${title} - ${post.title}`
      }
    } else {
      title = `${title} - Post by ${post.user_handle} on ${dayjs
        .utc(post.created_at)
        .format('L')}`
    }
  }

  return (
    <section className={cx('orbit-page-post', className)}>
      <Head>
        <title>{title}</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />

      <div className="orbit-page-post__feed">
        {shouldDisplay && (!!orbit || !!author) && (
          <PostContent
            className="orbit-page-post__feed-post"
            post={post}
            commentsLoading={commentsLoading}
            comments={comments}
            commentsCount={totalComments}
            onAddComment={() => setCommentModalOpen(true)}
            onDeleteComment={(postId, commentId) =>
              postActionDeleteComment(
                profile?.user_id || '',
                commentId,
                postId,
                session?.access_token,
                dispatch
              )
            }
          />
        )}
      </div>
      {shouldDisplay && !!orbit && (
        <aside className="orbit-page-post__sidebar">
          <InfoCard
            title="About this community"
            titleImageUrl={orbit.avatar_uri}
            innerHTML={orbit.description_html}
            actions={[
              {
                title: 'Post a comment',
                href: `/feed/${post.post_id}/new-comment`,
                button: 'default',
                action: (e) => {
                  e.preventDefault()
                  setCommentModalOpen(true)
                },
              },
            ]}
          >
            Created {dayjs.utc(orbit.created_at).format('MMM DD, YYYY')}
          </InfoCard>
        </aside>
      )}
      {shouldDisplay && !!author && (
        <aside className="orbit-page-post__sidebar">
          <InfoCard
            author={author}
            innerHTML={author.intro_html}
            actions={[
              {
                title: 'Post a comment',
                href: `/feed/${post.post_id}/new-comment`,
                button: 'default',
              },
            ]}
          >
            Joined {dayjs.utc(author.created_at).format('MMM DD, YYYY')}
          </InfoCard>
        </aside>
      )}
      {post && (
        <NewCommentModal
          open={commentModalOpen}
          onClose={() => setCommentModalOpen(false)}
          postId={post.post_id}
        />
      )}
    </section>
  )
}
