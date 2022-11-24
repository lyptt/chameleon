import {
  Children,
  cloneElement,
  isValidElement,
  ReactNode,
  useState,
} from 'react'
import cx from 'classnames'
import Nav from '@/components/molecules/Nav'
import AuthContext, {
  buildAuthContext,
  IAuthContext,
} from '@/components/organisms/AuthContext'
import DefaultActionsDelegator from '@/components/organisms/DefaultActionsDelegator'
import { FeedProvider } from '@/components/organisms/FeedContext'
import { ProfileProvider } from '@/components/organisms/ProfileContext'
import MobileNav from '@/components/molecules/MobileNav'
import { PostProvider } from '@/components/organisms/PostContext'
import {
  ThemeContext,
  ThemeProvider,
} from '@/components/organisms/ThemeContext'
import { useRouter } from 'next/router'
import Toolbox from '@/components/molecules/Toolbox'
import { UserProvider } from '@/components/organisms/UserContext'

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
      className: cx('chameleon-main__content'),
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
                <main className={cx('chameleon-main', theme)}>
                  {isBuiltInRoute && (
                    <>
                      <div
                        className="chameleon-main__spacer-left"
                        aria-hidden="true"
                      ></div>
                      <div className="chameleon-main-side-nav" />
                      {childrenWithClassname}
                      <div className="chameleon-main-nav" />
                      <div className="chameleon-main-mobile-nav" />
                      <div
                        className="chameleon-main__spacer-right"
                        aria-hidden="true"
                      ></div>
                    </>
                  )}
                  {!isBuiltInRoute && (
                    <>
                      <div
                        className="chameleon-main__spacer-left"
                        aria-hidden="true"
                      ></div>
                      <Toolbox className="chameleon-main-side-nav" />
                      {childrenWithClassname}
                      <Nav className="chameleon-main-nav" />
                      <MobileNav className="chameleon-main-mobile-nav" />
                      <div
                        className="chameleon-main__spacer-right"
                        aria-hidden="true"
                      ></div>
                    </>
                  )}
                </main>
              </UserProvider>
            </PostProvider>
          </FeedProvider>
        </ProfileProvider>
      </ThemeProvider>
    </AuthContext.Provider>
  )
}
