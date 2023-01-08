import { IListResponse, ISearchResult, fetchSearchResult } from '@/core/api'
import React, { useReducer, createContext, useMemo, useContext } from 'react'
import retry from 'async-retry'

export enum SearchType {
  PublicFederated,
  Own,
  Friends,
  Orbit,
}

enum SearchActionType {
  RESET_SEARCH_STATE = 'RESET_SEARCH_STATE',
  CLEAR_SEARCH_STATE = 'CLEAR_SEARCH_STATE',
  SET_SEARCH_TERM = 'SET_SEARCH_TERM',
  REFRESH_SEARCH_LOADING = 'REFRESH_SEARCH_LOADING',
  REFRESH_SEARCH_ERROR = 'REFRESH_SEARCH_ERROR',
  REFRESH_SEARCH_LOADED = 'REFRESH_SEARCH_LOADED',
}

interface SearchAction {
  type: SearchActionType
  results?: IListResponse<ISearchResult>
  error?: any
  searchTerm?: string
  page?: number
}

export async function searchActionReset(
  dispatch: React.Dispatch<SearchAction>
) {
  dispatch({
    type: SearchActionType.RESET_SEARCH_STATE,
  })
}

export async function searchActionClear(
  dispatch: React.Dispatch<SearchAction>
) {
  dispatch({
    type: SearchActionType.CLEAR_SEARCH_STATE,
  })
}

export async function searchActionSetSearchTerm(
  searchTerm: string | undefined,
  dispatch: React.Dispatch<SearchAction>
) {
  dispatch({
    type: SearchActionType.SET_SEARCH_TERM,
    searchTerm,
  })
}

export async function searchActionLoadResult(
  searchTerm: string,
  page: number,
  authToken: string | undefined,
  dispatch: React.Dispatch<SearchAction>
) {
  dispatch({
    type: SearchActionType.REFRESH_SEARCH_LOADING,
    searchTerm,
    page,
  })

  try {
    await retry(
      async () => {
        const results = await fetchSearchResult(searchTerm, authToken, page, 20)
        dispatch({
          type: SearchActionType.REFRESH_SEARCH_LOADED,
          results,
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
      type: SearchActionType.REFRESH_SEARCH_ERROR,
      error,
    })
  }
}

export interface ISearchState {
  results: ISearchResult[]
  initialLoadComplete: boolean
  loading: boolean
  loadingFailed: boolean
  page: number
  totalPages?: number
  noMorePages: boolean
  searchTerm?: string
  searchedTerm?: string
}

const initialState: ISearchState = {
  results: [],
  initialLoadComplete: false,
  loading: false,
  loadingFailed: false,
  page: 0,
  noMorePages: false,
  searchTerm: undefined,
}

export const SearchContext = createContext(
  {} as any as { state: ISearchState; dispatch: React.Dispatch<SearchAction> }
)

const reducer = (state: ISearchState, action: SearchAction): ISearchState => {
  switch (action.type) {
    case SearchActionType.RESET_SEARCH_STATE:
      return {
        ...initialState,
        searchTerm: state.searchTerm,
      }
    case SearchActionType.CLEAR_SEARCH_STATE:
      return {
        ...initialState,
        initialLoadComplete: true,
      }
    case SearchActionType.SET_SEARCH_TERM:
      return {
        ...state,
        searchTerm: action.searchTerm,
      }
    case SearchActionType.REFRESH_SEARCH_LOADING:
      return {
        ...state,
        loading: true,
        loadingFailed: false,
        initialLoadComplete: true,
        searchedTerm: action.searchTerm,
        results:
          action.searchTerm === state.searchedTerm &&
          (action.page || 0) > state.page
            ? state.results
            : [],
      }
    case SearchActionType.REFRESH_SEARCH_ERROR:
      return {
        ...state,
        loading: false,
        loadingFailed: true,
      }
    case SearchActionType.REFRESH_SEARCH_LOADED: {
      const results = [...state.results, ...(action.results?.data ?? [])]
      return {
        ...state,
        loading: false,
        loadingFailed: false,
        results,
        totalPages: action.results?.total_pages ?? state.totalPages,
        noMorePages: results.length >= (action.results?.total_items || 0),
        page: action.results?.page || 0,
      }
    }
    default:
      return state
  }
}

export const SearchProvider = ({ children }: any) => {
  const [state, dispatch] = useReducer(reducer, initialState) // (**)
  const contextValue = useMemo(() => {
    return { state, dispatch }
  }, [state, dispatch])
  return (
    <SearchContext.Provider value={contextValue}>
      {children}
    </SearchContext.Provider>
  )
}

export function useSearch() {
  return useContext(SearchContext)
}
