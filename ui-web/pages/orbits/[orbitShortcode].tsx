import { feedActionLoadFeed, useFeed } from '@/components/organisms/FeedContext'
import Head from 'next/head'
import {
  Dispatch,
  HTMLAttributes,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import SideNav from '@/components/molecules/SideNav'
import PostCard from '@/components/atoms/PostCard'
import { debounce } from 'lodash'
import { useRouter } from 'next/router'
import {
  orbitActionJoinOrbit,
  orbitActionLeaveOrbit,
  orbitActionLoadOrbit,
  useOrbits,
} from '@/components/organisms/OrbitContext'
import Masthead from '@/components/atoms/Masthead'
import InfoCard, { InfoCardAction } from '@/components/atoms/InfoCard'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import { useProfile } from '@/components/organisms/ProfileContext'

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

export default function OrbitPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const router = useRouter()
  const { session } = useAuth()
  const { state: orbitState, dispatch: orbitDispatch } = useOrbits()
  const { state, dispatch } = useFeed()
  const {
    state: { profile },
  } = useProfile()
  const { orbit } = orbitState
  const {
    loading,
    loadingFailed,
    feed,
    page,
    noMorePages,
    orbit: feedOrbit,
  } = state
  const orbitShortcode = (router.query.orbitShortcode || '') as string

  const [listInView, setListInView] = useState(false)

  useEffect(() => {
    if (!orbit || orbit.shortcode !== orbitShortcode) {
      orbitActionLoadOrbit(orbitShortcode, session?.access_token, orbitDispatch)
    }
  }, [orbitShortcode, session, orbitDispatch, dispatch])

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
      !orbit ||
      orbit.shortcode !== orbitShortcode ||
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
        orbit,
        false,
        dispatch
      )
    }
  }, [
    loading,
    feed,
    session,
    noMorePages,
    page,
    listInView,
    orbit,
    orbitShortcode,
  ])

  const actions = useMemo(() => {
    if (!orbit) {
      return []
    }

    const values: InfoCardAction[] = [
      {
        title: 'Post Something',
        href: `/orbits/${orbitShortcode}/new-post`,
        button: 'default',
      },
    ]

    if (orbit.joined) {
      values.push({
        title: 'Leave Orbit',
        button: 'outline',
        action: (e) => {
          e.preventDefault()
          if (!profile?.handle) {
            return
          }

          orbitActionLeaveOrbit(
            profile.handle,
            orbit.orbit_id,
            session?.access_token,
            orbitDispatch
          )
        },
      })
    } else {
      values.push({
        title: 'Join Orbit',
        button: 'outline',
        action: (e) => {
          e.preventDefault()
          if (!profile?.handle) {
            return
          }

          orbitActionJoinOrbit(
            profile.handle,
            orbit.orbit_id,
            session?.access_token,
            orbitDispatch
          )
        },
      })
    }

    if (orbit.moderating) {
      values.push({
        title: 'Moderation Tools',
        button: 'outline',
        href: `/orbits/${orbitShortcode}/moderation`,
      })
    }

    return values
  }, [orbit, session, profile])

  return (
    <section className={cx('orbit-page-orbit', className)}>
      <Head>
        <title>
          {!!orbit ? `Orbit - o/${orbit.shortcode}` : ''}
          {!orbit ? 'Orbit' : ''}
        </title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />
      {orbit &&
        orbit.shortcode === orbitShortcode &&
        feedOrbit?.orbit_id === orbit.orbit_id && (
          <>
            <div className="orbit-page-orbit__feed">
              <Masthead orbit={orbit} />
              {!loading &&
                !loadingFailed &&
                feed.map((post) => (
                  <PostCard
                    className="orbit-page-orbit__feed-post"
                    key={post.post_id}
                    post={post}
                    hideOrbitInformation
                  />
                ))}
            </div>
            <aside className="orbit-page-orbit__sidebar">
              <InfoCard
                title="About this community"
                titleImageUrl={orbit.avatar_uri}
                innerHTML={orbit.description_html}
                actions={actions}
              >
                Created {dayjs.utc(orbit.created_at).format('MMM DD, YYYY')}
              </InfoCard>
            </aside>
          </>
        )}
    </section>
  )
}
