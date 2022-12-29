import { fetchUserOrbit, fetchUserOrbits, IOrbit } from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'

enum OrbitActionType {
  REFRESH_USER_ORBITS_LOADING = 'REFRESH_USER_ORBITS_LOADING',
  REFRESH_USER_ORBITS_ERROR = 'REFRESH_USER_ORBITS_ERROR',
  REFRESH_USER_ORBITS_LOADED = 'REFRESH_USER_ORBITS_LOADED',
  REFRESH_USER_ORBIT_LOADING = 'REFRESH_USER_ORBIT_LOADING',
  REFRESH_USER_ORBIT_ERROR = 'REFRESH_USER_ORBIT_ERROR',
  REFRESH_USER_ORBIT_LOADED = 'REFRESH_USER_ORBIT_LOADED',
}

interface OrbitAction {
  type: OrbitActionType
  data?: any
  error?: any
}

export async function orbitActionLoadUserOrbits(
  handle: string,
  authToken: string,
  dispatch: React.Dispatch<OrbitAction>
) {
  dispatch({
    type: OrbitActionType.REFRESH_USER_ORBITS_LOADING,
  })

  try {
    const orbits = await fetchUserOrbits(handle, authToken, 0)
    dispatch({
      type: OrbitActionType.REFRESH_USER_ORBITS_LOADED,
      data: orbits.data,
    })
  } catch (error) {
    dispatch({
      type: OrbitActionType.REFRESH_USER_ORBITS_ERROR,
      error,
    })
  }
}

export async function orbitActionLoadOrbit(
  shortcode: string,
  authToken: string | undefined,
  dispatch: React.Dispatch<OrbitAction>
) {
  dispatch({
    type: OrbitActionType.REFRESH_USER_ORBIT_LOADING,
  })

  try {
    const orbits = await fetchUserOrbit(shortcode, authToken)
    dispatch({
      type: OrbitActionType.REFRESH_USER_ORBIT_LOADED,
      data: orbits.data,
    })
  } catch (error) {
    dispatch({
      type: OrbitActionType.REFRESH_USER_ORBIT_ERROR,
      error,
    })
  }
}

export interface IOrbitState {
  orbits?: IOrbit[]
  loading: boolean
  loadingFailed: boolean
  loadingOrbit: boolean
  loadingOrbitFailed: boolean
  orbit?: IOrbit
}

const initialState: IOrbitState = {
  loading: false,
  loadingFailed: false,
  loadingOrbit: false,
  loadingOrbitFailed: false,
  orbit: undefined,
}

export const OrbitContext = createContext(
  {} as any as { state: IOrbitState; dispatch: React.Dispatch<OrbitAction> }
)

const reducer = (state: IOrbitState, action: OrbitAction): IOrbitState => {
  switch (action.type) {
    case OrbitActionType.REFRESH_USER_ORBITS_LOADING:
      return {
        ...state,
        loading: true,
        loadingFailed: false,
      }
    case OrbitActionType.REFRESH_USER_ORBITS_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case OrbitActionType.REFRESH_USER_ORBITS_LOADED:
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        orbits: action.data,
      }
    case OrbitActionType.REFRESH_USER_ORBIT_LOADING:
      return {
        ...state,
        loadingOrbit: true,
        loadingOrbitFailed: false,
      }
    case OrbitActionType.REFRESH_USER_ORBIT_ERROR:
      return {
        ...state,
        loadingOrbit: false,
        loadingOrbitFailed: true,
      }
    case OrbitActionType.REFRESH_USER_ORBIT_LOADED:
      return {
        ...state,
        loadingOrbit: false,
        loadingOrbitFailed: false,
        orbit: action.data,
      }
    default:
      return state
  }
}

export const OrbitProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <OrbitContext.Provider value={contextValue}>
      {children}
    </OrbitContext.Provider>
  )
}

export function useOrbits() {
  return useContext(OrbitContext)
}
