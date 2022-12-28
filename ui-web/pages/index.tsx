import { useFeed } from '@/components/organisms/FeedContext'
import Head from 'next/head'
import { HTMLAttributes } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import { usePost } from '@/components/organisms/PostContext'
import SideNav from '@/components/molecules/SideNav'

export default function HomePage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useFeed()
  const { dispatch: postDispatch } = usePost()
  const {
    loading,
    loadingFailed,
    feed,
    submitting,
    submittingImageProgress,
    page,
    noMorePages,
  } = state

  return (
    <section className={cx('orbit-page-home', className)}>
      <Head>
        <title>Orbit</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <SideNav />
    </section>
  )
}
