import { fetchProfile, IProfile } from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'

enum ProfileActionType {
  REFRESH_USER_PROFILE_LOADING = 'REFRESH_USER_PROFILE_LOADING',
  REFRESH_USER_PROFILE_ERROR = 'REFRESH_USER_PROFILE_ERROR',
  REFRESH_USER_PROFILE_LOADED = 'REFRESH_USER_PROFILE_LOADED',
}

interface ProfileAction {
  type: ProfileActionType
  data?: any
  error?: any
}

export async function profileActionLoadProfile(
  authToken: string,
  dispatch: React.Dispatch<ProfileAction>
) {
  dispatch({
    type: ProfileActionType.REFRESH_USER_PROFILE_LOADING,
  })

  try {
    const profile = await fetchProfile(authToken)
    dispatch({
      type: ProfileActionType.REFRESH_USER_PROFILE_LOADED,
      data: profile,
    })
  } catch (error) {
    dispatch({
      type: ProfileActionType.REFRESH_USER_PROFILE_ERROR,
      error,
    })
  }
}

export interface IProfileState {
  profile?: IProfile
  loading: boolean
  loadingFailed: boolean
}

const initialState: IProfileState = {
  loading: false,
  loadingFailed: false,
}

export const ProfileContext = createContext(
  {} as any as { state: IProfileState; dispatch: React.Dispatch<ProfileAction> }
)

const reducer = (
  state: IProfileState,
  action: ProfileAction
): IProfileState => {
  switch (action.type) {
    case ProfileActionType.REFRESH_USER_PROFILE_LOADING:
      return {
        ...state,
        loading: true,
        loadingFailed: false,
      }
    case ProfileActionType.REFRESH_USER_PROFILE_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case ProfileActionType.REFRESH_USER_PROFILE_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        profile: action.data,
      }
    default:
      return state
  }
}

export const ProfileProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <ProfileContext.Provider value={contextValue}>
      {children}
    </ProfileContext.Provider>
  )
}

export function useProfile() {
  return useContext(ProfileContext)
}
