import Head from 'next/head'
import cx from 'classnames'

export default function ProfilePage() {
  return (
    <section className="chameleon-page-profile">
      <Head>
        <title>Chameleon</title>
        <meta
          name="description"
          content="Welcome to Chameleon, your place to share photos of your life with family and friends in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
    </section>
  )
}
