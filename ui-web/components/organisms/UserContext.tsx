import {
  fetchProfile,
  fetchUserFeed,
  fetchUserProfile,
  IListResponse,
  IPost,
  IProfile,
} from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'
import retry from 'async-retry'

enum UserActionType {
  REFRESH_USER_PROFILE_LOADING = 'REFRESH_USER_PROFILE_LOADING',
  REFRESH_USER_PROFILE_ERROR = 'REFRESH_USER_PROFILE_ERROR',
  REFRESH_USER_PROFILE_LOADED = 'REFRESH_USER_PROFILE_LOADED',
  REFRESH_USER_POSTS_LOADING = 'REFRESH_USER_POSTS_LOADING',
  REFRESH_USER_POSTS_ERROR = 'REFRESH_USER_POSTS_ERROR',
  REFRESH_USER_POSTS_LOADED = 'REFRESH_USER_POSTS_LOADED',
}

interface UserAction {
  type: UserActionType
  data?: any
  feedData?: IListResponse<IPost>
  error?: any
}

export async function userActionLoadProfile(
  handle: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<UserAction>
) {
  dispatch({
    type: UserActionType.REFRESH_USER_PROFILE_LOADING,
  })

  try {
    const profile = await fetchUserProfile(handle, authToken)
    dispatch({
      type: UserActionType.REFRESH_USER_PROFILE_LOADED,
      data: profile,
    })
  } catch (error) {
    dispatch({
      type: UserActionType.REFRESH_USER_PROFILE_ERROR,
      error,
    })
  }
}

export async function userActionLoadFeed(
  handle: string,
  page: number,
  authToken: string | undefined,
  dispatch: React.Dispatch<UserAction>
) {
  dispatch({
    type: UserActionType.REFRESH_USER_POSTS_LOADING,
  })

  try {
    await retry(
      async () => {
        const result = await fetchUserFeed(handle, authToken, page, 20)
        dispatch({
          type: UserActionType.REFRESH_USER_POSTS_LOADED,
          feedData: result,
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
      type: UserActionType.REFRESH_USER_POSTS_ERROR,
      error,
    })
  }
}

export interface IUserState {
  profile?: IProfile
  loading: boolean
  loadingFailed: boolean
  data: IPost[]
  postsLoading: boolean
  postsLoadingFailed: boolean
  page: number
  totalPages?: number
  noMorePages: boolean
  initialLoadComplete: boolean
}

const initialState: IUserState = {
  loading: false,
  loadingFailed: false,
  data: [],
  postsLoading: false,
  postsLoadingFailed: false,
  page: 0,
  noMorePages: false,
  initialLoadComplete: false,
}

export const UserContext = createContext(
  {} as any as { state: IUserState; dispatch: React.Dispatch<UserAction> }
)

const reducer = (state: IUserState, action: UserAction): IUserState => {
  switch (action.type) {
    case UserActionType.REFRESH_USER_PROFILE_LOADING:
      return {
        ...initialState,
        loading: true,
        loadingFailed: false,
      }
    case UserActionType.REFRESH_USER_PROFILE_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case UserActionType.REFRESH_USER_PROFILE_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        profile: action.data,
      }
    case UserActionType.REFRESH_USER_POSTS_LOADING:
      return {
        ...state,
        postsLoading: true,
        postsLoadingFailed: false,
        initialLoadComplete: true,
      }
    case UserActionType.REFRESH_USER_POSTS_ERROR:
      return {
        ...state,
        postsLoading: false,
        postsLoadingFailed: true,
      }
    case UserActionType.REFRESH_USER_POSTS_LOADED:
      const feedData = action.feedData?.data ?? []
      const feed = [...state.data, ...feedData]

      return {
        ...state,
        postsLoading: false,
        postsLoadingFailed: false,
        data: feed,
        totalPages: action.feedData?.total_pages ?? state.totalPages,
        noMorePages:
          feed.length >= (action.feedData?.total_items || 0) ||
          !feedData.length,
        page: action.feedData?.page || 0,
      }
    default:
      return state
  }
}

export const UserProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <UserContext.Provider value={contextValue}>{children}</UserContext.Provider>
  )
}

export function useUser() {
  return useContext(UserContext)
}
