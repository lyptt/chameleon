import { feedActionLoadFeed, useFeed } from '@/components/organisms/FeedContext'
import Head from 'next/head'
import { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import SideNav from '@/components/molecules/SideNav'
import PostCard from '@/components/atoms/PostCard'
import WelcomeCard from '@/components/atoms/WelcomeCard'
import { debounce } from 'lodash'

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

export default function FriendsFeedPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useFeed()
  const { loading, loadingFailed, feed, page, noMorePages } = state

  const [listInView, setListInView] = useState(false)

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
      feedActionLoadFeed(
        page + 1,
        session?.access_token,
        undefined,
        true,
        dispatch
      )
    }
  }, [loading, feed, session, noMorePages, page, listInView])

  return (
    <section className={cx('orbit-page-feed-friends', className)}>
      <Head>
        <title>Orbit - Friends and Followers</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />
      <div className="orbit-page-feed-friends__feed">
        {!loading &&
          !loadingFailed &&
          feed.map((post) => (
            <PostCard
              className="orbit-page-feed-friends__feed-post"
              key={post.post_id}
              post={post}
            />
          ))}
      </div>
      <aside className="orbit-page-feed-friends__sidebar">
        <WelcomeCard />
      </aside>
    </section>
  )
}
