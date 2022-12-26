import {
  Children,
  cloneElement,
  isValidElement,
  ReactNode,
  useState,
} from 'react'
import cx from 'classnames'
import AuthContext, {
  buildAuthContext,
  IAuthContext,
} from '@/components/organisms/AuthContext'
import DefaultActionsDelegator from '@/components/organisms/DefaultActionsDelegator'
import { FeedProvider } from '@/components/organisms/FeedContext'
import { ProfileProvider } from '@/components/organisms/ProfileContext'
import { PostProvider } from '@/components/organisms/PostContext'
import { ThemeProvider } from '@/components/organisms/ThemeContext'
import { useRouter } from 'next/router'
import { UserProvider } from '@/components/organisms/UserContext'
import NavBar from '../molecules/NavBar'

interface MainLayoutProps {
  defaultAuthContext?: IAuthContext
  children: ReactNode
  theme?: string
}

function getDocumentCookie() {
  if (typeof window === 'undefined') {
    return ''
  }

  return window.document.cookie
}

export default function MainLayout({
  defaultAuthContext,
  children,
  theme,
}: MainLayoutProps) {
  const { route } = useRouter()

  const isBuiltInRoute =
    route.startsWith('/_') || route === '/404' || route === '/error'

  const [authContext] = useState(
    defaultAuthContext ?? buildAuthContext(getDocumentCookie())
  )

  const childrenWithClassname = Children.map(children, (child) => {
    if (!isValidElement(child)) {
      return child
    }

    return cloneElement(child, {
      className: cx('orbit-main__content'),
    } as any)
  })

  return (
    <AuthContext.Provider value={authContext}>
      <ThemeProvider value={{ theme }}>
        <ProfileProvider>
          <FeedProvider>
            <PostProvider>
              <UserProvider>
                <DefaultActionsDelegator />
                <main className={cx('orbit-main', theme)}>
                  <NavBar isBuiltInRoute={isBuiltInRoute} />
                  {isBuiltInRoute && <>{childrenWithClassname}</>}
                  {!isBuiltInRoute && <>{childrenWithClassname}</>}
                </main>
              </UserProvider>
            </PostProvider>
          </FeedProvider>
        </ProfileProvider>
      </ThemeProvider>
    </AuthContext.Provider>
  )
}
