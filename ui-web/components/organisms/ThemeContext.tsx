import { createContext, useContext } from 'react'

export const ThemeContext = createContext({ theme: '' })

export function useTheme() {
  return useContext(ThemeContext)
}
