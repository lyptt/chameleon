import { HTMLProps } from 'react'
import cx from 'classnames'
import {
  searchActionClear,
  searchActionReset,
  searchActionSetSearchTerm,
  useSearch,
} from '../organisms/SearchContext'
import { useRouter } from 'next/router'

export default function SearchBar({
  className,
  ...props
}: HTMLProps<HTMLInputElement>) {
  const { state, dispatch } = useSearch()
  const { searchTerm } = state
  const router = useRouter()

  return (
    <input
      className={cx('orbit-search-bar', className)}
      {...props}
      placeholder="Search"
      value={searchTerm || ''}
      onChange={(e) => {
        searchActionSetSearchTerm(e.target.value || undefined, dispatch)
      }}
      onKeyUp={(e) => {
        if (e.key === 'Enter') {
          if (router.route === '/search') {
            if (!searchTerm) {
              router.push('/search')
              return
            }
            router.push(`/search?term=${searchTerm}`)
          } else if (searchTerm) {
            router.push(`/search?term=${searchTerm}`)
          } else {
            router.push(`/search`)
          }
        }
      }}
    />
  )
}
