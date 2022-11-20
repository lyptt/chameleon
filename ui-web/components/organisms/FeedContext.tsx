import {
  fetchFederatedFeed,
  fetchOwnFeed,
  submitPost,
  IListResponse,
  INewPost,
  IPost,
  submitPostImage,
  getJobStatus,
  JobStatus,
  fetchPost,
  likePost,
  unlikePost,
  createPostComment,
} from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'
import retry from 'async-retry'

export enum FeedType {
  PublicFederated,
  Own,
}

enum FeedActionType {
  REFRESH_FEED_LOADING = 'REFRESH_FEED_LOADING',
  REFRESH_FEED_ERROR = 'REFRESH_FEED_ERROR',
  REFRESH_FEED_LOADED = 'REFRESH_FEED_LOADED',
  SUBMIT_POST_SENDING_METADATA = 'SUBMIT_POST_SENDING_METADATA',
  SUBMIT_POST_SENDING_IMAGE = 'SUBMIT_POST_SENDING_IMAGE',
  SUBMIT_POST_SENDING_IMAGE_PROGRESS = 'SUBMIT_POST_SENDING_IMAGE_PROGRESS',
  SUBMIT_POST_WAITING_FOR_JOB = 'SUBMIT_POST_WAITING_FOR_JOB',
  SUBMIT_POST_ERROR = 'SUBMIT_POST_ERROR',
  SUBMIT_POST_COMPLETED = 'SUBMIT_POST_COMPLETED',
  SUBMIT_POST_DISMISS_ERROR = 'SUBMIT_POST_DISMISS_ERROR',
  UPDATE_POST_LIKED = 'UPDATE_POST_LIKED',
  UPDATE_POST_COMMENTED = 'UPDATE_POST_COMMENTED',
}

interface FeedAction {
  type: FeedActionType
  feedData?: IListResponse<IPost>
  newPostMetadata?: INewPost
  newPostFile?: File
  newPostJobId?: string
  newPost?: IPost
  error?: any
  feedType?: FeedType
  progress?: number
  postId?: string
  liked?: boolean
  comment?: string
}

export async function feedActionLoadFeed(
  page: number,
  authToken: string | undefined,
  dispatch: React.Dispatch<FeedAction>
) {
  dispatch({
    type: FeedActionType.REFRESH_FEED_LOADING,
    feedType: authToken ? FeedType.Own : FeedType.PublicFederated,
  })

  try {
    await retry(
      async () => {
        const result = authToken
          ? await fetchOwnFeed(authToken, page, 5)
          : await fetchFederatedFeed(page, 5)
        dispatch({
          type: FeedActionType.REFRESH_FEED_LOADED,
          feedData: result,
          feedType: authToken ? FeedType.Own : FeedType.PublicFederated,
        })
      },
      {
        retries: 5,
        factor: 2,
        randomize: true,
      }
    )
  } catch (error) {
    dispatch({
      type: FeedActionType.REFRESH_FEED_ERROR,
      error,
      feedType: authToken ? FeedType.Own : FeedType.PublicFederated,
    })
  }
}

export async function feedActionSubmitPost(
  post: INewPost,
  file: File,
  authToken: string | undefined,
  dispatch: React.Dispatch<FeedAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: FeedActionType.SUBMIT_POST_SENDING_METADATA,
    newPostMetadata: post,
  })

  try {
    const createdRecord = await submitPost(post, authToken)

    dispatch({
      type: FeedActionType.SUBMIT_POST_SENDING_IMAGE,
      newPostFile: file,
    })

    const job = await submitPostImage(
      createdRecord.id,
      file,
      authToken,
      (progress) =>
        dispatch({
          type: FeedActionType.SUBMIT_POST_SENDING_IMAGE_PROGRESS,
          progress,
        })
    )

    dispatch({
      type: FeedActionType.SUBMIT_POST_WAITING_FOR_JOB,
      newPostJobId: job.job_id,
    })

    await retry(
      async () => {
        const res = await getJobStatus(job.job_id, authToken)

        if (res.status !== JobStatus.Done && res.status !== JobStatus.Failed) {
          throw new Error('Not complete yet')
        } else if (res.status === JobStatus.Failed) {
          return dispatch({
            type: FeedActionType.SUBMIT_POST_ERROR,
          })
        } else {
          const post = await fetchPost(res.completion_record_id!, authToken)
          return dispatch({
            type: FeedActionType.SUBMIT_POST_COMPLETED,
            newPost: post.data,
          })
        }
      },
      {
        retries: 100,
        factor: 1.2,
        maxRetryTime: 1000 * 60 * 30,
        maxTimeout: 1500,
        randomize: true,
      }
    )
  } catch (error) {
    dispatch({
      type: FeedActionType.SUBMIT_POST_ERROR,
      error,
    })
  }
}

export async function feedActionUpdatePostLiked(
  liked: boolean,
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<FeedAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: FeedActionType.UPDATE_POST_LIKED,
    liked,
    postId,
  })

  try {
    liked
      ? await likePost(postId, authToken)
      : await unlikePost(postId, authToken)
  } catch (err) {
    console.error(err)
  }
}

export async function feedActionAddPostComment(
  comment: string,
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<FeedAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: FeedActionType.UPDATE_POST_COMMENTED,
    comment,
    postId,
  })

  try {
    await createPostComment(comment, postId, authToken)
  } catch (err) {
    console.error(err)
  }
}

export async function feedActionAddPostSoftComment(
  comment: string,
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<FeedAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: FeedActionType.UPDATE_POST_COMMENTED,
    comment,
    postId,
  })
}

export interface IFeedState {
  feed: IPost[]
  initialLoadComplete: boolean
  loading: boolean
  loadingFailed: boolean
  submitting: boolean
  submittingMetadata: boolean
  submittingImage: boolean
  submittingImageProgress?: number
  submittingFailed: boolean
  submittingPost?: INewPost | null
  submittingFile?: File | null
  submittingJobId?: string | null
  page: number
  totalPages?: number
  noMorePages: boolean
  type: FeedType
}

const initialState: IFeedState = {
  feed: [],
  initialLoadComplete: false,
  loading: false,
  loadingFailed: false,
  submitting: false,
  submittingMetadata: false,
  submittingImage: false,
  submittingImageProgress: undefined,
  submittingFailed: false,
  page: 0,
  noMorePages: false,
  type: FeedType.PublicFederated,
}

export const FeedContext = createContext(
  {} as any as { state: IFeedState; dispatch: React.Dispatch<FeedAction> }
)

const reducer = (state: IFeedState, action: FeedAction): IFeedState => {
  switch (action.type) {
    case FeedActionType.REFRESH_FEED_LOADING:
      return {
        ...state,
        loading: true,
        loadingFailed: false,
        initialLoadComplete: true,
        type: action.feedType ?? state.type,
      }
    case FeedActionType.REFRESH_FEED_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
        type: action.feedType ?? state.type,
      }
    case FeedActionType.REFRESH_FEED_LOADED: {
      const feed = [...state.feed, ...(action.feedData?.data ?? [])]
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        feed,
        totalPages: action.feedData?.total_pages ?? state.totalPages,
        type: action.feedType ?? state.type,
        noMorePages: feed.length >= (action.feedData?.total_items || 0),
        page: action.feedData?.page || 0,
      }
    }
    case FeedActionType.SUBMIT_POST_SENDING_METADATA:
      return {
        ...state,
        submitting: true,
        submittingMetadata: true,
        submittingPost: action.newPostMetadata,
      }
    case FeedActionType.SUBMIT_POST_SENDING_IMAGE:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: true,
        submittingImageProgress: 0,
        submittingFile: action.newPostFile,
      }
    case FeedActionType.SUBMIT_POST_SENDING_IMAGE_PROGRESS:
      return {
        ...state,
        submittingImageProgress:
          action.progress !== undefined && action.progress < 1
            ? action.progress
            : undefined,
      }
    case FeedActionType.SUBMIT_POST_WAITING_FOR_JOB:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingPost: null,
        submittingFile: null,
        submittingJobId: action.newPostJobId,
      }
    case FeedActionType.SUBMIT_POST_COMPLETED:
      if (!action.newPost) {
        return {
          ...state,
          submitting: false,
          submittingMetadata: false,
          submittingImage: false,
          submittingImageProgress: undefined,
          submittingPost: null,
          submittingFile: null,
          submittingJobId: null,
          submittingFailed: false,
        }
      }

      return {
        ...state,
        submitting: false,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingPost: null,
        submittingFile: null,
        submittingJobId: null,
        submittingFailed: false,
        feed: [action.newPost, ...state.feed],
      }
    case FeedActionType.SUBMIT_POST_ERROR:
      return {
        ...state,
        submitting: false,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingPost: null,
        submittingFile: null,
        submittingJobId: null,
        submittingFailed: true,
      }
    case FeedActionType.SUBMIT_POST_DISMISS_ERROR:
      return {
        ...state,
        submittingFailed: false,
      }
    case FeedActionType.UPDATE_POST_LIKED: {
      if (action.liked === undefined || action.postId === undefined) {
        return state
      }
      const postIdx = state.feed.findIndex(
        (post) => post.post_id === action.postId
      )
      if (postIdx === -1) {
        return state
      }

      const feed = [...state.feed]
      feed[postIdx] = {
        ...feed[postIdx],
        liked: action.liked,
        likes: action.liked ? feed[postIdx].likes + 1 : feed[postIdx].likes - 1,
      }

      return { ...state, feed }
    }
    case FeedActionType.UPDATE_POST_COMMENTED: {
      if (action.comment === undefined) {
        return state
      }
      const postIdx = state.feed.findIndex(
        (post) => post.post_id === action.postId
      )
      if (postIdx === -1) {
        return state
      }

      const feed = [...state.feed]
      feed[postIdx] = {
        ...feed[postIdx],
        comments: feed[postIdx].comments + 1,
      }

      return { ...state, feed }
    }
    default:
      return state
  }
}

export const FeedProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <FeedContext.Provider value={contextValue}>{children}</FeedContext.Provider>
  )
}

export function useFeed() {
  return useContext(FeedContext)
}
