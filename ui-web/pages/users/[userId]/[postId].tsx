import PostCard from '@/components/molecules/PostCard'
import Head from 'next/head'
import { HTMLAttributes, useCallback, useEffect, useState } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import { debounce } from 'lodash'
import ActivityIndicator from '@/components/quarks/ActivityIndicator'
import { IComment, IPost } from '@/core/api'
import StatusBar from '@/components/molecules/StatusBar'
import { IoChevronBack } from 'react-icons/io5'
import { useRouter } from 'next/router'
import {
  usePost,
  postActionLoadPost,
  postActionLoadComments,
  postActionUpdateCommentLiked,
  postActionSelectComment,
  postActionSelectPost,
} from '@/components/organisms/PostContext'
import Comment from '@/components/atoms/Comment'

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

export default function PostPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { query, replace, asPath } = useRouter()
  const { userId, postId, from } = query

  const backUri = (from as string) || '/'

  const { session } = useAuth()
  const { state, dispatch } = usePost()
  const {
    loading,
    loadingFailed,
    commentsLoading,
    commentsLoadingFailed,
    post,
    initialCommentLoadComplete,
    comments,
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

  useEffect(() => {
    if (!post && !loading && !loadingFailed) {
      postActionLoadPost(postId as string, session?.access_token, dispatch)
      return
    }

    if (post?.post_id !== postId && !loading && !loadingFailed) {
      postActionLoadPost(postId as string, session?.access_token, dispatch)
      return
    }

    if (!initialCommentLoadComplete && !!post) {
      postActionLoadComments(post.post_id, 0, session?.access_token, dispatch)
    }
  }, [
    dispatch,
    initialCommentLoadComplete,
    post,
    session,
    loading,
    loadingFailed,
  ])

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
  }, [comments, setListInView])

  useEffect(() => {
    if (
      loading ||
      loadingFailed ||
      !post ||
      !postId ||
      noMorePages ||
      !listInView
    ) {
      return
    }

    // HACK: We're not getting an accurate indication that we're at the bottom from the IntersectionObserver. It's triggering
    //       at strange times, so we also need to check scroll position manually.
    if (determineScrollPercentage() >= 0.75) {
      postActionLoadComments(
        postId as string,
        page + 1,
        session?.access_token,
        dispatch
      )
    }
  }, [loading, post, postId, session, noMorePages, page, listInView])

  if (!userId || !postId) {
    replace('/404')
    return <></>
  }

  const handleCommentLiked = (comment: IComment) => {
    postActionUpdateCommentLiked(
      !comment.liked,
      comment.comment_id,
      postId as string,
      session?.access_token,
      dispatch
    )
  }

  const handlePostReplied = (post: IPost) => {
    postActionSelectPost(post, dispatch)
  }

  const handleCommentReplied = (comment: IComment) => {
    postActionSelectComment(comment, dispatch)
  }

  return (
    <section className={cx('chameleon-page-post', className)}>
      <Head>
        <title>Chameleon</title>
        <meta
          name="description"
          content="Welcome to Chameleon, your place to share photos of your life with family and friends in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <StatusBar className="chameleon-page-post__status-bar" href={backUri}>
        <IoChevronBack />
        <span>Back</span>
      </StatusBar>
      {!loading && !!post && (
        <PostCard
          className={cx('chameleon-page-post__post', {
            'chameleon-page-post__post--comments-available':
              comments.length > 0,
          })}
          post={post}
          linkToPost={false}
          handlePostReplied={handlePostReplied}
          backUri={asPath}
        />
      )}
      {!commentsLoading && comments.length > 0 && (
        <div className="chameleon-page-post__comments">
          {comments.map((comment) => (
            <Comment
              key={comment.comment_id}
              comment={comment}
              handleCommentLiked={handleCommentLiked}
              handleCommentReplied={handleCommentReplied}
              backUri={asPath}
            />
          ))}
        </div>
      )}
      {comments.length > 0 &&
        !noMorePages &&
        initialCommentLoadComplete &&
        !commentsLoadingFailed && (
          <ActivityIndicator className="chameleon-post__indicator" />
        )}
      {commentsLoadingFailed && (
        <p className="chameleon-post__indicator">
          We had trouble fetching more comments, please try again later.
        </p>
      )}
    </section>
  )
}
