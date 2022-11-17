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
    const result = authToken
      ? await fetchOwnFeed(authToken, page)
      : await fetchFederatedFeed(page)
    dispatch({
      type: FeedActionType.REFRESH_FEED_LOADED,
      feedData: result,
      feedType: authToken ? FeedType.Own : FeedType.PublicFederated,
    })
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
      async (bail) => {
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
      }
    )
  } catch (error) {
    dispatch({
      type: FeedActionType.SUBMIT_POST_ERROR,
      error,
    })
  }
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
    case FeedActionType.REFRESH_FEED_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        feed: [...state.feed, ...(action.feedData?.data ?? [])],
        totalPages: action.feedData?.total_pages ?? state.totalPages,
        type: action.feedType ?? state.type,
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
