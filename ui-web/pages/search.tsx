import Head from 'next/head'
import React, { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import SideNav from '@/components/molecules/SideNav'
import { debounce } from 'lodash'
import { useRouter } from 'next/router'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import WelcomeCard from '@/components/atoms/WelcomeCard'
import {
  searchActionLoadResult,
  useSearch,
} from '@/components/organisms/SearchContext'
import InfoCard from '@/components/atoms/InfoCard'

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

export default function SearchPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const router = useRouter()
  const { session } = useAuth()
  const { state, dispatch } = useSearch()
  const { loading, loadingFailed, results, searchTerm, page, noMorePages } =
    state
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
  }, [results, setListInView])

  useEffect(() => {
    if (
      !searchTerm ||
      loading ||
      loadingFailed ||
      !results.length ||
      noMorePages ||
      !listInView
    ) {
      return
    }

    // HACK: We're not getting an accurate indication that we're at the bottom from the IntersectionObserver. It's triggering
    //       at strange times, so we also need to check scroll position manually.
    if (determineScrollPercentage() >= 0.75) {
      searchActionLoadResult(
        searchTerm,
        page + 1,
        session?.access_token,
        dispatch
      )
    }
  }, [loading, searchTerm, session, noMorePages, page, listInView, results])

  return (
    <section className={cx('orbit-page-searech', className)}>
      <Head>
        <title>Orbit - Search</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />
      <div className="orbit-page-search__feed">
        {!loading &&
          !loadingFailed &&
          results.map((result) => (
            <React.Fragment
              key={result.orbit?.orbit_id || result.user?.user_id || '0'}
            >
              <InfoCard author={result.user} orbit={result.orbit} slim />
            </React.Fragment>
          ))}
      </div>
      <aside className="orbit-page-search__sidebar">
        <WelcomeCard />
      </aside>
    </section>
  )
}
