import UserButton from '@/components/atoms/UserButton'
import PostCard from '@/components/molecules/PostCard'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  fetchFederatedFeed,
  fetchOwnFeed,
  IListResponse,
  IPost,
} from '@/core/api'
import Head from 'next/head'
import { useEffect, useState } from 'react'
import classNames from './Home.module.css'

export default function Home() {
  const { authenticated, session } = useAuth()
  const [posts, setPosts] = useState<IListResponse<IPost> | undefined>()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(false)

  useEffect(() => {
    if (loading || posts || error) {
      return
    }

    ;(async () => {
      setLoading(true)
      setError(false)
      try {
        const result =
          authenticated && !!session
            ? await fetchOwnFeed(session.access_token, 0)
            : await fetchFederatedFeed(0)
        setPosts(result)
        setLoading(false)
      } catch {
        setLoading(false)
        setError(true)
      }
    })()
  }, [authenticated, session, loading, posts])

  return (
    <section className={classNames.container}>
      <Head>
        <title>Chameleon</title>
        <meta
          name="description"
          content="Welcome to Chameleon, your place to share photos of your life with family and friends in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className={classNames.feed}>
        {!loading &&
          posts &&
          posts.data.map((post) => (
            <PostCard
              key={post.post_id}
              className={classNames.post}
              post={post}
            />
          ))}
      </div>
    </section>
  )
}
