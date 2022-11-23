import { createContext, useContext } from 'react'

export const ThemeContext = createContext({} as any as { theme: string })

export const ThemeProvider = ({ theme, children }: any) => {
  return (
    <ThemeContext.Provider value={{ theme }}>{children}</ThemeContext.Provider>
  )
}

export function useTheme() {
  return useContext(ThemeContext)
}
