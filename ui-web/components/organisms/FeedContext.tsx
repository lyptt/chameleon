import {
  fetchFederatedFeed,
  fetchOwnFeed,
  IListResponse,
  INewPost,
  IPost,
  likePost,
  unlikePost,
  createPostComment,
  IOrbit,
  fetchOrbitFeed,
  IOrbitProfile,
  fetchOwnFriendsFeed,
} from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'
import retry from 'async-retry'

export enum FeedType {
  PublicFederated,
  Own,
  Friends,
  Orbit,
}

enum FeedActionType {
  RESET_FEED_STATE = 'RESET_FEED_STATE',
  REFRESH_FEED_LOADING = 'REFRESH_FEED_LOADING',
  REFRESH_FEED_ERROR = 'REFRESH_FEED_ERROR',
  REFRESH_FEED_LOADED = 'REFRESH_FEED_LOADED',
  UPDATE_POST_LIKED = 'UPDATE_POST_LIKED',
  UPDATE_POST_COMMENTED = 'UPDATE_POST_COMMENTED',
}

interface FeedAction {
  type: FeedActionType
  feedData?: IListResponse<IPost>
  error?: any
  feedType?: FeedType
  postId?: string
  liked?: boolean
  joined?: boolean
  comment?: string
  orbit?: IOrbitProfile
}

export async function feedActionReset(dispatch: React.Dispatch<FeedAction>) {
  dispatch({
    type: FeedActionType.RESET_FEED_STATE,
  })
}

export async function feedActionLoadFeed(
  page: number,
  authToken: string | undefined,
  orbit: IOrbitProfile | undefined,
  friendsOnly: boolean,
  dispatch: React.Dispatch<FeedAction>
) {
  let feedType: FeedType
  if (orbit) {
    feedType = FeedType.Orbit
  } else if (friendsOnly) {
    feedType = FeedType.Friends
  } else {
    feedType = authToken ? FeedType.Own : FeedType.PublicFederated
  }

  dispatch({
    type: FeedActionType.REFRESH_FEED_LOADING,
    feedType,
    orbit,
  })

  try {
    await retry(
      async () => {
        let result: IListResponse<IPost>
        if (orbit) {
          result = await fetchOrbitFeed(orbit.shortcode, authToken, page, 20)
        } else if (friendsOnly) {
          if (!authToken) {
            throw new Error('Unauthorized')
          }
          result = await fetchOwnFriendsFeed(authToken, page, 20)
        } else {
          result = authToken
            ? await fetchOwnFeed(authToken, page, 20)
            : await fetchFederatedFeed(page, 20)
        }
        dispatch({
          type: FeedActionType.REFRESH_FEED_LOADED,
          feedData: result,
          feedType,
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
      feedType,
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
  page: number
  totalPages?: number
  noMorePages: boolean
  type: FeedType
  orbit?: IOrbitProfile
}

const initialState: IFeedState = {
  feed: [],
  initialLoadComplete: false,
  loading: false,
  loadingFailed: false,
  page: 0,
  noMorePages: false,
  type: FeedType.PublicFederated,
}

export const FeedContext = createContext(
  {} as any as { state: IFeedState; dispatch: React.Dispatch<FeedAction> }
)

const reducer = (state: IFeedState, action: FeedAction): IFeedState => {
  switch (action.type) {
    case FeedActionType.RESET_FEED_STATE:
      return {
        ...initialState,
      }
    case FeedActionType.REFRESH_FEED_LOADING:
      return {
        ...state,
        loading: true,
        loadingFailed: false,
        initialLoadComplete: true,
        type: action.feedType ?? state.type,
        feed:
          !!action.feedType && action.feedType !== state.type ? [] : state.feed,
        orbit: action.orbit,
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
