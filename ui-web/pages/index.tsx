import PostCard from '@/components/molecules/PostCard'
import { useFeed } from '@/components/organisms/FeedContext'
import Progress from '@/components/quarks/Progress'
import Head from 'next/head'
import classNames from './Home.module.css'

export default function Home() {
  const { state } = useFeed()

  const { loading, feed, submitting, submittingImageProgress } = state

  console.log(submittingImageProgress)

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
          feed &&
          feed.map((post) => (
            <PostCard
              key={post.post_id}
              className={classNames.post}
              post={post}
            />
          ))}
        {submitting && (
          <Progress
            className={classNames.progress}
            value={submittingImageProgress}
            max={1}
          />
        )}
      </div>
    </section>
  )
}
