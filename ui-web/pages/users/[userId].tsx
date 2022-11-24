import Head from 'next/head'
import { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import { debounce } from 'lodash'
import ActivityIndicator from '@/components/quarks/ActivityIndicator'
import StatusBar from '@/components/molecules/StatusBar'
import { IoChevronBack } from 'react-icons/io5'
import { useRouter } from 'next/router'
import {
  userActionLoadFeed,
  userActionLoadProfile,
  useUser,
} from '@/components/organisms/UserContext'
import UserProfileTile from '@/components/molecules/UserProfileTile'
import Link from 'next/link'
import Config from '@/core/config'
import { LazyImage } from '@/components/atoms/LazyImage'

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
  const { query, replace } = useRouter()
  const { userId } = query

  const { session } = useAuth()
  const { state, dispatch } = useUser()
  const {
    loading,
    loadingFailed,
    postsLoading,
    postsLoadingFailed,
    data,
    profile,
    page,
    noMorePages,
    initialLoadComplete,
  } = state
  const [listInView, setListInView] = useState(false)

  // Triggered on each scroll event, but debounced to 500ms as to not affect performance
  const checkScrollPosition = useCallback(
    debounce(() => {
      setListInView(determineScrollPercentage() >= 0.75)
    }, 500),
    [setListInView]
  )

  useEffect(() => {
    if (!profile && !loading && !loadingFailed) {
      userActionLoadProfile(userId as string, session?.access_token, dispatch)
      return
    }

    if (profile?.handle !== userId && !loading && !loadingFailed) {
      userActionLoadProfile(userId as string, session?.access_token, dispatch)
      return
    }

    if (!initialLoadComplete && !!profile) {
      userActionLoadFeed(
        userId as string,
        page,
        session?.access_token,
        dispatch
      )
    }
  }, [dispatch, initialLoadComplete, profile, session, loading, loadingFailed])

  // Add a scroll listener to listen for user scroll events so that we know when we've reached near the bottom of the page.
  useEffect(() => {
    const handler = checkScrollPosition
    document.addEventListener('scroll', handler, { passive: true })

    return () => {
      document.removeEventListener('scroll', handler)
    }
  }, [checkScrollPosition])

  // If we've added / removed more comments recently, update the listInView value so that we can reset the flag for subsequent
  // scroll events.
  useEffect(() => {
    setListInView(determineScrollPercentage() >= 0.75)
  }, [data, setListInView])

  useEffect(() => {
    if (
      loading ||
      loadingFailed ||
      !profile ||
      !userId ||
      noMorePages ||
      !listInView
    ) {
      return
    }

    // HACK: We're not getting an accurate indication that we're at the bottom from the IntersectionObserver. It's triggering
    //       at strange times, so we also need to check scroll position manually.
    if (determineScrollPercentage() >= 0.75) {
      userActionLoadFeed(
        userId as string,
        page + 1,
        session?.access_token,
        dispatch
      )
    }
  }, [loading, profile, userId, session, noMorePages, page, listInView])

  if (!userId) {
    replace('/404')
    return <></>
  }

  const lastRowStartIdx = Math.floor(data.length / 3) * 3

  return (
    <section className={cx('chameleon-page-user', className)}>
      <Head>
        <title>Chameleon</title>
        <meta
          name="description"
          content="Welcome to Chameleon, your place to share photos of your life with family and friends in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <StatusBar className="chameleon-page-user__status-bar">
        <IoChevronBack />
        <span>Back</span>
      </StatusBar>
      {!loading && profile && (
        <UserProfileTile
          profile={profile}
          feedAvailable={!postsLoading && data.length > 0}
          className="chameleon-page-user__profile"
        />
      )}
      {!postsLoading && data.length > 0 && (
        <div className="chameleon-page-user__posts">
          {data.map((post, i) => (
            <Link
              key={post.post_id}
              className={cx('chameleon-page-user__post', {
                'chameleon-page-user__post--left': i % 3 === 0,
                'chameleon-page-user__post--middle': i % 3 === 1,
                'chameleon-page-user__post--right': i % 3 === 2,
                'chameleon-page-user__post--last': i >= lastRowStartIdx,
              })}
              href={
                post.uri.indexOf('http') === 0
                  ? post.uri
                  : `${Config.fqdn}/users/${post.user_handle}/${post.uri}`
              }
            >
              <LazyImage
                className="chameleon-page-user__post-image"
                contentClassName="chameleon-page-user__post-image-content"
                blurhash={post.content_blurhash}
                srcSet={`${Config.cdn}/${post.content_image_uri_large} ${post.content_width_large}w, ${Config.cdn}/${post.content_image_uri_medium} ${post.content_width_medium}w, ${Config.cdn}/${post.content_image_uri_small} ${post.content_width_small}w`}
                src={`${Config.cdn}/${post.content_image_uri_medium}`}
              />
            </Link>
          ))}
        </div>
      )}
      {data.length > 0 &&
        !noMorePages &&
        initialLoadComplete &&
        !postsLoadingFailed && (
          <ActivityIndicator className="chameleon-post__indicator" />
        )}
    </section>
  )
}
