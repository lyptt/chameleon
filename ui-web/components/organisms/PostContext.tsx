import {
  createPostComment,
  createPostCommentLike,
  deletePostCommentLike,
  fetchOrbit,
  fetchPost,
  fetchPostComments,
  fetchUserProfile,
  IComment,
  IListResponse,
  IObjectResponse,
  IOrbit,
  IPost,
  IProfile,
} from '@/core/api'
import retry from 'async-retry'
import React, { useReducer, createContext, useMemo, useContext } from 'react'

enum PostActionType {
  REFRESH_POST_LOADING = 'REFRESH_POST_LOADING',
  REFRESH_POST_ERROR = 'REFRESH_POST_ERROR',
  REFRESH_POST_LOADED = 'REFRESH_POST_LOADED',
  REFRESH_ORBIT_LOADING = 'REFRESH_ORBIT_LOADING',
  REFRESH_ORBIT_ERROR = 'REFRESH_ORBIT_ERROR',
  REFRESH_ORBIT_LOADED = 'REFRESH_ORBIT_LOADED',
  REFRESH_AUTHOR_LOADING = 'REFRESH_AUTHOR_LOADING',
  REFRESH_AUTHOR_ERROR = 'REFRESH_AUTHOR_ERROR',
  REFRESH_AUTHOR_LOADED = 'REFRESH_AUTHOR_LOADED',
  REFRESH_POST_COMMENTS_LOADING = 'REFRESH_POST_COMMENTS_LOADING',
  REFRESH_POST_COMMENTS_ERROR = 'REFRESH_POST_COMMENTS_ERROR',
  REFRESH_POST_COMMENTS_LOADED = 'REFRESH_POST_COMMENTS_LOADED',
  UPDATE_POST_LIKED = 'UPDATE_POST_LIKED',
  UPDATE_COMMENT_LIKED = 'UPDATE_COMMENT_LIKED',
  SELECT_COMMENT = 'SELECT_COMMENT',
  SELECT_POST = 'SELECT_POST',
  DESELECT = 'DESELECT',
  DISMISS_POST = 'DISMISS_POST',
}

interface PostAction {
  type: PostActionType
  data?: IObjectResponse<IPost>
  orbitData?: IObjectResponse<IOrbit>
  authorData?: IProfile
  comments?: IListResponse<IComment>
  error?: any
  liked?: boolean
  commentId?: string
  comment?: IComment
  post?: IPost
}

export async function postActionLoadPost(
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({
    type: PostActionType.REFRESH_POST_LOADING,
  })

  try {
    const post = await fetchPost(postId, authToken)
    dispatch({
      type: PostActionType.REFRESH_POST_LOADED,
      data: post,
    })
  } catch (error) {
    dispatch({
      type: PostActionType.REFRESH_POST_ERROR,
      error,
    })
  }
}

export async function postActionLoadOrbit(
  orbitShortcode: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({
    type: PostActionType.REFRESH_ORBIT_LOADING,
  })

  try {
    const post = await fetchOrbit(orbitShortcode, authToken)
    dispatch({
      type: PostActionType.REFRESH_ORBIT_LOADED,
      orbitData: post,
    })
  } catch (error) {
    dispatch({
      type: PostActionType.REFRESH_ORBIT_ERROR,
      error,
    })
  }
}

export async function postActionLoadAuthor(
  userHandle: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({
    type: PostActionType.REFRESH_AUTHOR_LOADING,
  })

  try {
    const post = await fetchUserProfile(userHandle, authToken)
    dispatch({
      type: PostActionType.REFRESH_AUTHOR_LOADED,
      authorData: post,
    })
  } catch (error) {
    dispatch({
      type: PostActionType.REFRESH_AUTHOR_ERROR,
      error,
    })
  }
}

export async function postActionLoadComments(
  postId: string,
  page: number,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({
    type: PostActionType.REFRESH_POST_COMMENTS_LOADING,
  })

  try {
    await retry(
      async () => {
        const result = await fetchPostComments(postId, authToken, page, 20)
        dispatch({
          type: PostActionType.REFRESH_POST_COMMENTS_LOADED,
          comments: result,
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
      type: PostActionType.REFRESH_POST_COMMENTS_ERROR,
      error,
    })
  }
}

export function postActionDismissPost(dispatch: React.Dispatch<PostAction>) {
  return dispatch({
    type: PostActionType.DISMISS_POST,
  })
}

export function postActionUpdateLiked(
  liked: boolean,
  dispatch: React.Dispatch<PostAction>
) {
  return dispatch({
    type: PostActionType.UPDATE_POST_LIKED,
    liked,
  })
}

export async function postActionSelectComment(
  comment: IComment,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({ type: PostActionType.SELECT_COMMENT, comment })
}

export async function postActionSelectPost(
  post: IPost,
  dispatch: React.Dispatch<PostAction>
) {
  dispatch({ type: PostActionType.SELECT_POST, post })
}

export async function postActionDeselect(dispatch: React.Dispatch<PostAction>) {
  dispatch({ type: PostActionType.DESELECT })
}

export async function postActionAddPostComment(
  comment: string,
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  if (!authToken) {
    return
  }

  dispatch({ type: PostActionType.DESELECT })

  try {
    await createPostComment(comment, postId, authToken)
    await postActionLoadComments(postId, 0, authToken, dispatch)
  } catch (err) {
    console.error(err)
  }
}

export async function postActionUpdateCommentLiked(
  liked: boolean,
  commentId: string,
  postId: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<PostAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: PostActionType.UPDATE_COMMENT_LIKED,
    liked: liked,
    commentId,
  })

  try {
    liked
      ? await createPostCommentLike(postId, commentId, authToken)
      : await deletePostCommentLike(postId, commentId, authToken)
  } catch (err) {
    console.error(err)
  }
}

export interface IPostState {
  post?: IPost
  author?: IProfile
  orbit?: IOrbit
  comments: IComment[]
  loading: boolean
  loadingFailed: boolean
  initialCommentLoadComplete: boolean
  commentsLoading: boolean
  commentsLoadingFailed: boolean
  authorLoading: boolean
  authorLoadingFailed: boolean
  orbitLoading: boolean
  orbitLoadingFailed: boolean
  page: number
  totalPages?: number
  totalComments?: number
  noMorePages: boolean
  selectedComment?: IComment
  selectedPost?: IPost
}

const initialState: IPostState = {
  comments: [],
  loading: false,
  loadingFailed: false,
  initialCommentLoadComplete: false,
  commentsLoading: false,
  commentsLoadingFailed: false,
  authorLoading: false,
  authorLoadingFailed: false,
  orbitLoading: false,
  orbitLoadingFailed: false,
  page: 0,
  noMorePages: false,
  totalComments: undefined,
  selectedComment: undefined,
  selectedPost: undefined,
}

export const PostContext = createContext(
  {} as any as { state: IPostState; dispatch: React.Dispatch<PostAction> }
)

const reducer = (state: IPostState, action: PostAction): IPostState => {
  switch (action.type) {
    case PostActionType.REFRESH_POST_LOADING:
      return {
        ...initialState,
        loading: true,
      }
    case PostActionType.REFRESH_POST_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case PostActionType.REFRESH_POST_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        post: action.data?.data,
      }
    case PostActionType.REFRESH_ORBIT_LOADING:
      return {
        ...state,
        loading: true,
      }
    case PostActionType.REFRESH_ORBIT_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case PostActionType.REFRESH_ORBIT_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        orbit: action.orbitData?.data,
      }
    case PostActionType.REFRESH_AUTHOR_LOADING:
      return {
        ...state,
        loading: true,
      }
    case PostActionType.REFRESH_AUTHOR_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case PostActionType.REFRESH_AUTHOR_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        author: action.authorData,
      }
    case PostActionType.DISMISS_POST:
      return {
        ...initialState,
      }
    case PostActionType.REFRESH_POST_COMMENTS_LOADING:
      return {
        ...state,
        commentsLoading: true,
        commentsLoadingFailed: false,
        initialCommentLoadComplete: true,
      }
    case PostActionType.REFRESH_POST_COMMENTS_ERROR:
      return {
        ...state,
        commentsLoading: false,
        commentsLoadingFailed: true,
      }
    case PostActionType.REFRESH_POST_COMMENTS_LOADED: {
      const comments =
        state.page === 0 && state.comments.length
          ? action.comments?.data ?? []
          : [...state.comments, ...(action.comments?.data ?? [])]

      return {
        ...state,
        commentsLoading: false,
        commentsLoadingFailed: false,
        comments,
        totalPages: action.comments?.total_pages ?? state.totalPages,
        noMorePages: comments.length >= (action.comments?.total_items || 0),
        page: action.comments?.page || 0,
        totalComments: action.comments?.total_items ?? state.totalComments,
      }
    }
    case PostActionType.UPDATE_POST_LIKED: {
      if (!state.post) {
        return state
      }

      return {
        ...state,
        post: {
          ...state.post,
          liked: action.liked ?? state.post.liked,
          likes: action.liked ? state.post.likes + 1 : state.post.likes - 1,
        },
      }
    }
    case PostActionType.UPDATE_COMMENT_LIKED: {
      const index = state.comments.findIndex(
        (comment) => comment.comment_id === action.commentId
      )
      if (index === -1) {
        return state
      }

      const newComments = [...state.comments]
      newComments[index] = {
        ...newComments[index],
        liked: action.liked,
        likes: action.liked
          ? newComments[index].likes + 1
          : newComments[index].likes - 1,
      }

      return { ...state, comments: newComments }
    }
    case PostActionType.SELECT_COMMENT:
      return {
        ...state,
        selectedComment: action.comment,
      }
    case PostActionType.SELECT_POST:
      return {
        ...state,
        selectedPost: action.post,
      }
    case PostActionType.DESELECT:
      return {
        ...state,
        selectedComment: undefined,
        selectedPost: undefined,
      }
    default:
      return state
  }
}

export const PostProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <PostContext.Provider value={contextValue}>{children}</PostContext.Provider>
  )
}

export function usePost() {
  return useContext(PostContext)
}
