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
import classNames from './MainLayout.module.css'
import MobileNav from '@/components/molecules/MobileNav'

interface MainLayoutProps {
  defaultAuthContext?: IAuthContext
  children: ReactNode
}

function getDocumentCookie() {
  if (typeof window === 'undefined') {
    return ''
  }

  return window.document.cookie
}

export default function MainLayout(props: MainLayoutProps) {
  const { defaultAuthContext, children } = props
  const [authContext] = useState(
    defaultAuthContext ?? buildAuthContext(getDocumentCookie())
  )

  const childrenWithClassname = Children.map(children, (child) => {
    if (!isValidElement(child)) {
      return child
    }

    return cloneElement(child, {
      className: cx('chameleon-main__content', classNames.content),
    } as any)
  })

  return (
    <AuthContext.Provider value={authContext}>
      <ProfileProvider>
        <FeedProvider>
          <DefaultActionsDelegator />
          <main className={cx('chameleon-main', classNames.layout)}>
            <Nav className={cx('chameleon-main-nav', classNames.nav)} />
            <MobileNav
              className={cx('chameleon-main-mobile-nav', classNames.mobileNav)}
            />
            {childrenWithClassname}
          </main>
        </FeedProvider>
      </ProfileProvider>
    </AuthContext.Provider>
  )
}
