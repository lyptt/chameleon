import {
  fetchOrbit,
  fetchOrbitById,
  fetchPost,
  fetchProfile,
  fetchUserProfile,
  getJobStatus,
  INewOrbit,
  INewPost,
  INewProfile,
  IObjectResponse,
  IOrbit,
  IPost,
  IProfile,
  JobStatus,
  submitOrbit,
  submitOrbitImage,
  submitPost,
  submitPostImage,
  submitProfile,
  submitProfileImage,
} from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'
import retry from 'async-retry'

enum CreateActionType {
  INITIALIZE = 'INITIALIZE',
  REFRESH_ORBIT_LOADING = 'REFRESH_ORBIT_LOADING',
  REFRESH_ORBIT_ERROR = 'REFRESH_ORBIT_ERROR',
  REFRESH_ORBIT_LOADED = 'REFRESH_ORBIT_LOADED',
  DISMISS = 'DISMISS',
  SUBMIT_POST_SENDING_METADATA = 'SUBMIT_POST_SENDING_METADATA',
  SUBMIT_POST_SENDING_IMAGE = 'SUBMIT_POST_SENDING_IMAGE',
  SUBMIT_POST_SENDING_IMAGE_PROGRESS = 'SUBMIT_POST_SENDING_IMAGE_PROGRESS',
  SUBMIT_POST_WAITING_FOR_JOB = 'SUBMIT_POST_WAITING_FOR_JOB',
  SUBMIT_POST_ERROR = 'SUBMIT_POST_ERROR',
  SUBMIT_POST_COMPLETED = 'SUBMIT_POST_COMPLETED',
  SUBMIT_POST_DISMISS_ERROR = 'SUBMIT_POST_DISMISS_ERROR',
  SUBMIT_ORBIT_SENDING_METADATA = 'SUBMIT_ORBIT_SENDING_METADATA',
  SUBMIT_ORBIT_SENDING_IMAGE = 'SUBMIT_ORBIT_SENDING_IMAGE',
  SUBMIT_ORBIT_SENDING_IMAGE_PROGRESS = 'SUBMIT_ORBIT_SENDING_IMAGE_PROGRESS',
  SUBMIT_ORBIT_ERROR = 'SUBMIT_ORBIT_ERROR',
  SUBMIT_ORBIT_COMPLETED = 'SUBMIT_ORBIT_COMPLETED',
  SUBMIT_ORBIT_DISMISS_ERROR = 'SUBMIT_ORBIT_DISMISS_ERROR',
  SUBMIT_PROFILE_SENDING_METADATA = 'SUBMIT_PROFILE_SENDING_METADATA',
  SUBMIT_PROFILE_SENDING_IMAGE = 'SUBMIT_PROFILE_SENDING_IMAGE',
  SUBMIT_PROFILE_SENDING_IMAGE_PROGRESS = 'SUBMIT_PROFILE_SENDING_IMAGE_PROGRESS',
  SUBMIT_PROFILE_ERROR = 'SUBMIT_PROFILE_ERROR',
  SUBMIT_PROFILE_COMPLETED = 'SUBMIT_PROFILE_COMPLETED',
  SUBMIT_PROFILE_DISMISS_ERROR = 'SUBMIT_PROFILE_DISMISS_ERROR',
}

interface CreateAction {
  type: CreateActionType
  orbitData?: IObjectResponse<IOrbit>
  error?: any
  post?: IPost
  newPostMetadata?: INewPost
  newPostFiles?: File[]
  newPostJobId?: string
  newPost?: IPost
  newOrbitMetadata?: INewOrbit
  newOrbitFiles?: File[]
  newOrbitJobId?: string
  newOrbit?: IOrbit
  newProfileMetadata?: INewProfile
  newProfileFiles?: File[]
  newProfileJobId?: string
  newProfile?: IProfile
  progress?: number
}

export async function createActionInitialize(
  orbitShortcode: string | undefined,
  authToken: string | undefined,
  dispatch: React.Dispatch<CreateAction>
) {
  dispatch({
    type: CreateActionType.INITIALIZE,
  })

  if (!orbitShortcode) {
    return
  }

  dispatch({
    type: CreateActionType.REFRESH_ORBIT_LOADING,
  })

  try {
    const post = await fetchOrbit(orbitShortcode, authToken)
    dispatch({
      type: CreateActionType.REFRESH_ORBIT_LOADED,
      orbitData: post,
    })
  } catch (error) {
    dispatch({
      type: CreateActionType.REFRESH_ORBIT_ERROR,
      error,
    })
  }
}

export async function createActionSubmitPost(
  post: INewPost,
  files: File[],
  authToken: string | undefined,
  dispatch: React.Dispatch<CreateAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: CreateActionType.SUBMIT_POST_SENDING_METADATA,
    newPostMetadata: post,
  })

  try {
    const createdRecord = await submitPost(post, authToken)

    let job_id: string

    if ('id' in createdRecord) {
      dispatch({
        type: CreateActionType.SUBMIT_POST_SENDING_IMAGE,
        newPostFiles: files,
      })

      const job = await submitPostImage(
        createdRecord.id,
        files,
        authToken,
        (progress) =>
          dispatch({
            type: CreateActionType.SUBMIT_POST_SENDING_IMAGE_PROGRESS,
            progress,
          })
      )
      job_id = job.job_id
    } else {
      job_id = createdRecord.job_id
    }

    dispatch({
      type: CreateActionType.SUBMIT_POST_WAITING_FOR_JOB,
      newPostJobId: job_id,
    })

    await retry(
      async () => {
        const res = await getJobStatus(job_id, authToken)

        if (res.status !== JobStatus.Done && res.status !== JobStatus.Failed) {
          throw new Error('Not complete yet')
        } else if (res.status === JobStatus.Failed) {
          return dispatch({
            type: CreateActionType.SUBMIT_POST_ERROR,
          })
        } else {
          const post = await fetchPost(res.record_id!, authToken)
          return dispatch({
            type: CreateActionType.SUBMIT_POST_COMPLETED,
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
      type: CreateActionType.SUBMIT_POST_ERROR,
      error,
    })
  }
}

export async function createActionSubmitOrbit(
  orbit: INewOrbit,
  files: File[],
  authToken: string | undefined,
  dispatch: React.Dispatch<CreateAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: CreateActionType.SUBMIT_ORBIT_SENDING_METADATA,
    newOrbitMetadata: orbit,
  })

  try {
    const createdRecord = await submitOrbit(orbit, authToken)

    if (files.length) {
      dispatch({
        type: CreateActionType.SUBMIT_ORBIT_SENDING_IMAGE,
        newOrbitFiles: files,
      })

      await submitOrbitImage(createdRecord.id, files, authToken, (progress) =>
        dispatch({
          type: CreateActionType.SUBMIT_ORBIT_SENDING_IMAGE_PROGRESS,
          progress,
        })
      )
    }

    const newOrbit = await fetchOrbitById(createdRecord.id, authToken)

    return dispatch({
      type: CreateActionType.SUBMIT_ORBIT_COMPLETED,
      newOrbit: newOrbit.data,
    })
  } catch (error) {
    dispatch({
      type: CreateActionType.SUBMIT_ORBIT_ERROR,
      error,
    })
  }
}

export async function createActionSubmitProfile(
  currentProfile: IProfile,
  profile: INewProfile,
  files: File[],
  authToken: string | undefined,
  dispatch: React.Dispatch<CreateAction>
) {
  if (!authToken) {
    return
  }

  dispatch({
    type: CreateActionType.SUBMIT_PROFILE_SENDING_METADATA,
    newProfileMetadata: profile,
  })

  try {
    await submitProfile(currentProfile, profile, authToken)

    if (files.length) {
      dispatch({
        type: CreateActionType.SUBMIT_PROFILE_SENDING_IMAGE,
        newProfileFiles: files,
      })

      await submitProfileImage(files, authToken, (progress) =>
        dispatch({
          type: CreateActionType.SUBMIT_PROFILE_SENDING_IMAGE_PROGRESS,
          progress,
        })
      )
    }

    const newProfile = await fetchProfile(authToken)

    return dispatch({
      type: CreateActionType.SUBMIT_PROFILE_COMPLETED,
      newProfile: newProfile,
    })
  } catch (error) {
    dispatch({
      type: CreateActionType.SUBMIT_PROFILE_ERROR,
      error,
    })
  }
}

export function createActionDismiss(dispatch: React.Dispatch<CreateAction>) {
  return dispatch({
    type: CreateActionType.DISMISS,
  })
}

export interface ICreateState {
  initialized: boolean
  orbit?: IOrbit
  selectedPost?: IPost
  orbitLoading: boolean
  orbitLoadingFailed: boolean
  submitting: boolean
  submittingMetadata: boolean
  submittingImage: boolean
  submittingImageProgress?: number
  submittingFailed: boolean
  submittingPost?: INewPost | null
  submittingOrbit?: INewOrbit | null
  submittingProfile?: INewProfile | null
  submittingFiles?: File[] | null
  submittingJobId?: string | null
  submittedPost?: IPost
  submittedOrbit?: IOrbit
  submittedProfile?: IProfile
}

const initialState: ICreateState = {
  initialized: false,
  orbitLoading: false,
  orbitLoadingFailed: false,
  submitting: false,
  submittingMetadata: false,
  submittingImage: false,
  submittingImageProgress: undefined,
  submittingFailed: false,
}

export const CreateContext = createContext(
  {} as any as { state: ICreateState; dispatch: React.Dispatch<CreateAction> }
)

const reducer = (state: ICreateState, action: CreateAction): ICreateState => {
  switch (action.type) {
    case CreateActionType.INITIALIZE:
      return {
        ...state,
        initialized: true,
      }
    case CreateActionType.REFRESH_ORBIT_LOADING:
      return {
        ...state,
        orbitLoading: true,
      }
    case CreateActionType.REFRESH_ORBIT_ERROR:
      return {
        ...state,
        orbitLoading: false,
        orbitLoadingFailed: true,
      }
    case CreateActionType.REFRESH_ORBIT_LOADED:
      return {
        ...state,
        orbitLoading: false,
        orbitLoadingFailed: false,
        orbit: action.orbitData?.data,
      }
    case CreateActionType.DISMISS:
      return {
        ...initialState,
      }
    case CreateActionType.SUBMIT_POST_SENDING_METADATA:
      return {
        ...state,
        submitting: true,
        submittingMetadata: true,
        submittingPost: action.newPostMetadata,
      }
    case CreateActionType.SUBMIT_POST_SENDING_IMAGE:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: true,
        submittingImageProgress: 0,
        submittingFiles: action.newPostFiles,
      }
    case CreateActionType.SUBMIT_POST_SENDING_IMAGE_PROGRESS:
      return {
        ...state,
        submittingImageProgress:
          action.progress !== undefined && action.progress < 1
            ? action.progress
            : undefined,
      }
    case CreateActionType.SUBMIT_POST_WAITING_FOR_JOB:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingPost: null,
        submittingFiles: null,
        submittingJobId: action.newPostJobId,
      }
    case CreateActionType.SUBMIT_POST_COMPLETED:
      if (!action.newPost) {
        return {
          ...state,
          submitting: false,
          submittingMetadata: false,
          submittingImage: false,
          submittingImageProgress: undefined,
          submittingPost: null,
          submittingFiles: null,
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
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: false,
        submittedPost: action.newPost,
      }
    case CreateActionType.SUBMIT_POST_ERROR:
      return {
        ...state,
        submitting: false,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingPost: null,
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: true,
      }
    case CreateActionType.SUBMIT_POST_DISMISS_ERROR:
      return {
        ...state,
        submittingFailed: false,
      }
    case CreateActionType.SUBMIT_ORBIT_SENDING_METADATA:
      return {
        ...state,
        submitting: true,
        submittingMetadata: true,
        submittingOrbit: action.newOrbitMetadata,
      }
    case CreateActionType.SUBMIT_ORBIT_SENDING_IMAGE:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: true,
        submittingImageProgress: 0,
        submittingFiles: action.newOrbitFiles,
      }
    case CreateActionType.SUBMIT_ORBIT_SENDING_IMAGE_PROGRESS:
      return {
        ...state,
        submittingImageProgress:
          action.progress !== undefined && action.progress < 1
            ? action.progress
            : undefined,
      }
    case CreateActionType.SUBMIT_ORBIT_COMPLETED:
      if (!action.newOrbit) {
        return {
          ...state,
          submitting: false,
          submittingMetadata: false,
          submittingImage: false,
          submittingImageProgress: undefined,
          submittingOrbit: null,
          submittingFiles: null,
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
        submittingOrbit: null,
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: false,
        submittedOrbit: action.newOrbit,
      }
    case CreateActionType.SUBMIT_ORBIT_ERROR:
      return {
        ...state,
        submitting: false,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingOrbit: null,
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: true,
      }
    case CreateActionType.SUBMIT_ORBIT_DISMISS_ERROR:
      return {
        ...state,
        submittingFailed: false,
      }
    case CreateActionType.SUBMIT_PROFILE_SENDING_METADATA:
      return {
        ...state,
        submitting: true,
        submittingMetadata: true,
        submittingProfile: action.newProfileMetadata,
      }
    case CreateActionType.SUBMIT_PROFILE_SENDING_IMAGE:
      return {
        ...state,
        submitting: true,
        submittingMetadata: false,
        submittingImage: true,
        submittingImageProgress: 0,
        submittingFiles: action.newProfileFiles,
      }
    case CreateActionType.SUBMIT_PROFILE_SENDING_IMAGE_PROGRESS:
      return {
        ...state,
        submittingImageProgress:
          action.progress !== undefined && action.progress < 1
            ? action.progress
            : undefined,
      }
    case CreateActionType.SUBMIT_PROFILE_COMPLETED:
      if (!action.newProfile) {
        return {
          ...state,
          submitting: false,
          submittingMetadata: false,
          submittingImage: false,
          submittingImageProgress: undefined,
          submittingProfile: null,
          submittingFiles: null,
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
        submittingProfile: null,
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: false,
        submittedProfile: action.newProfile,
      }
    case CreateActionType.SUBMIT_PROFILE_ERROR:
      return {
        ...state,
        submitting: false,
        submittingMetadata: false,
        submittingImage: false,
        submittingImageProgress: undefined,
        submittingProfile: null,
        submittingFiles: null,
        submittingJobId: null,
        submittingFailed: true,
      }
    case CreateActionType.SUBMIT_PROFILE_DISMISS_ERROR:
      return {
        ...state,
        submittingFailed: false,
      }
    default:
      return state
  }
}

export const CreateProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <CreateContext.Provider value={contextValue}>
      {children}
    </CreateContext.Provider>
  )
}

export function useCreate() {
  return useContext(CreateContext)
}
