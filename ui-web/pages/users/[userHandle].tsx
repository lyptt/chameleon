import Head from 'next/head'
import { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import SideNav from '@/components/molecules/SideNav'
import PostCard from '@/components/atoms/PostCard'
import { debounce } from 'lodash'
import { useRouter } from 'next/router'
import InfoCard from '@/components/atoms/InfoCard'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import {
  userActionFollowUser,
  userActionLoadFeed,
  userActionLoadProfile,
  userActionUnfollowUser,
  useUser,
} from '@/components/organisms/UserContext'

dayjs.extend(dayjsUtc)

function determineScrollPercentage() {
  const documentHeight = Math.max(
    document.body.scrollHeight,
    document.documentElement.scrollHeight,
    document.body.offsetHeight,
    document.documentElement.offsetHeight,
    document.body.clientHeight,
    document.documentElement.clientHeight
  )

  const windowHeight =
    window.innerHeight ||
    (document.documentElement || document.body).clientHeight
  const scrollTop =
    window.pageYOffset ||
    (document.documentElement || document.body.parentNode || document.body)
      .scrollTop
  const trackLength = documentHeight - windowHeight

  return scrollTop / trackLength
}

export default function UserPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const router = useRouter()
  const { session } = useAuth()
  const { state, dispatch } = useUser()
  const {
    loading,
    loadingFailed,
    initialLoadComplete,
    data: feed,
    profile,
    page,
    noMorePages,
    stats,
  } = state
  const userHandle = (router.query.userHandle || '') as string | undefined

  const [listInView, setListInView] = useState(false)

  useEffect(() => {
    if (userHandle && !initialLoadComplete && !loading && !loadingFailed) {
      userActionLoadProfile(userHandle, session?.access_token, dispatch)
      userActionLoadFeed(userHandle, 0, session?.access_token, dispatch)
    }
  }, [userHandle, session, dispatch])

  // Triggered on each scroll event, but debounced to 500ms as to not affect performance
  const checkScrollPosition = useCallback(
    debounce(() => {
      setListInView(determineScrollPercentage() >= 0.75)
    }, 500),
    [setListInView]
  )

  // Add a scroll listener to listen for user scroll events so that we know when we've reached near the bottom of the page.
  useEffect(() => {
    const handler = checkScrollPosition
    document.addEventListener('scroll', handler, { passive: true })

    return () => {
      document.removeEventListener('scroll', handler)
    }
  }, [checkScrollPosition])

  // If we've added / removed more posts recently, update the listInView value so that we can reset the flag for subsequent
  // scroll events.
  useEffect(() => {
    setListInView(determineScrollPercentage() >= 0.75)
  }, [feed, setListInView])

  useEffect(() => {
    if (
      !profile ||
      !userHandle ||
      profile.handle !== userHandle ||
      loading ||
      loadingFailed ||
      !feed.length ||
      noMorePages ||
      !listInView
    ) {
      return
    }

    // HACK: We're not getting an accurate indication that we're at the bottom from the IntersectionObserver. It's triggering
    //       at strange times, so we also need to check scroll position manually.
    if (determineScrollPercentage() >= 0.75) {
      userActionLoadFeed(userHandle, page + 1, session?.access_token, dispatch)
    }
  }, [
    loading,
    feed,
    session,
    noMorePages,
    page,
    listInView,
    profile,
    userHandle,
  ])

  return (
    <section className={cx('user-page', className)}>
      <Head>
        <title>{profile ? `Orbit - u/${profile.handle}` : 'Orbit'}</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />
      {userHandle && profile && stats && profile.handle === userHandle && (
        <>
          <div className="user-page__feed">
            {!loading &&
              !loadingFailed &&
              feed.map((post) => (
                <PostCard
                  className="user-page__feed-post"
                  key={post.post_id}
                  post={post}
                  hideOrbitInformation
                />
              ))}
          </div>
          <aside className="user-page__sidebar">
            <InfoCard
              author={profile}
              innerHTML={profile.intro_html}
              actions={
                stats.user_is_you
                  ? [
                      {
                        title: 'Edit Profile',
                        href: '/profile/edit',
                        button: 'default',
                      },
                    ]
                  : [
                      {
                        title: stats.following_user ? 'Unfollow' : 'Follow',
                        action: stats.following_user
                          ? () =>
                              userActionUnfollowUser(
                                userHandle,
                                session?.access_token,
                                dispatch
                              )
                          : () =>
                              userActionFollowUser(
                                userHandle,
                                session?.access_token,
                                dispatch
                              ),
                        button: 'outline',
                      },
                    ]
              }
            >
              Joined {dayjs.utc(profile.created_at).format('MMM DD, YYYY')}
            </InfoCard>
          </aside>
        </>
      )}
    </section>
  )
}
