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
import { PostProvider } from '../organisms/PostContext'
import { ThemeContext, ThemeProvider } from '../organisms/ThemeContext'
import { useRouter } from 'next/router'

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
              <DefaultActionsDelegator />
              <main className={cx('chameleon-main', theme)}>
                {isBuiltInRoute && (
                  <>
                    <div
                      className={cx('chameleon-main__spacer-left')}
                      aria-hidden="true"
                    ></div>
                    <div className={cx('chameleon-main-side-nav')} />
                    {childrenWithClassname}
                    <div className={cx('chameleon-main-nav')} />
                    <div className={cx('chameleon-main-mobile-nav')} />
                    <div
                      className={cx('chameleon-main__spacer-right')}
                      aria-hidden="true"
                    ></div>
                  </>
                )}
                {!isBuiltInRoute && (
                  <>
                    <div
                      className={cx('chameleon-main__spacer-left')}
                      aria-hidden="true"
                    ></div>
                    <div className={cx('chameleon-main-side-nav')} />
                    {childrenWithClassname}
                    <Nav className={cx('chameleon-main-nav')} />
                    <MobileNav className={cx('chameleon-main-mobile-nav')} />
                    <div
                      className={cx('chameleon-main__spacer-right')}
                      aria-hidden="true"
                    ></div>
                  </>
                )}
              </main>
            </PostProvider>
          </FeedProvider>
        </ProfileProvider>
      </ThemeProvider>
    </AuthContext.Provider>
  )
}
