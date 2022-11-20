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
                <Nav className={cx('chameleon-main-nav')} />
                <MobileNav className={cx('chameleon-main-mobile-nav')} />
                {childrenWithClassname}
              </main>
            </PostProvider>
          </FeedProvider>
        </ProfileProvider>
      </ThemeProvider>
    </AuthContext.Provider>
  )
}
