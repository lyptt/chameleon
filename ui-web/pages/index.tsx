import PostCard from '@/components/molecules/PostCard'
import {
  feedActionLoadFeed,
  updatePostLiked,
  useFeed,
} from '@/components/organisms/FeedContext'
import Progress from '@/components/quarks/Progress'
import Head from 'next/head'
import { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import classNames from './Home.module.css'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import { debounce } from 'lodash'
import ActivityIndicator from '@/components/quarks/ActivityIndicator'
import { IPost } from '@/core/api'

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

  console.log(trackLength, scrollTop, scrollTop / trackLength)

  return scrollTop / trackLength
}

export default function Home({ className }: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useFeed()
  const {
    loading,
    loadingFailed,
    feed,
    submitting,
    submittingImageProgress,
    page,
    noMorePages,
  } = state
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
      feedActionLoadFeed(page + 1, session?.access_token, dispatch)
    }
  }, [loading, feed, session, noMorePages, page, listInView])

  const handlePostLiked = (post: IPost) => {
    updatePostLiked(!post.liked, post.post_id, session?.access_token, dispatch)
  }

  return (
    <section
      className={cx('chameleon-page-home', classNames.container, className)}
    >
      <Head>
        <title>Chameleon</title>
        <meta
          name="description"
          content="Welcome to Chameleon, your place to share photos of your life with family and friends in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className={cx('chameleon-feed', classNames.feed)}>
        {(!loading || feed.length > 0) &&
          feed &&
          feed.map((post) => (
            <PostCard
              key={post.post_id}
              className={cx('chameleon-feed__post', classNames.post)}
              post={post}
              handlePostLiked={handlePostLiked}
            />
          ))}
        {submitting && (
          <Progress
            className={cx('chameleon-home__progress', classNames.progress)}
            value={submittingImageProgress}
            max={1}
          />
        )}
        {feed.length > 0 && !noMorePages && !loadingFailed && (
          <ActivityIndicator
            className={cx('chameleon-home__indicator', classNames.indicator)}
          />
        )}
        {loadingFailed && (
          <p className={cx('chameleon-home__indicator', classNames.indicator)}>
            We had trouble fetching more posts, please try again later.
          </p>
        )}
      </div>
    </section>
  )
}
