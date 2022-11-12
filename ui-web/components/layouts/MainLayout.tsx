import { ReactNode, useState } from 'react'
import Nav from '../molecules/Nav'
import AuthContext, {
  buildAuthContext,
  IAuthContext,
} from '../organisms/AuthContext'
import DefaultActionsDelegator from '../organisms/DefaultActionsDelegator'
import { ProfileProvider } from '../organisms/ProfileContext'
import classNames from './MainLayout.module.css'

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

  return (
    <AuthContext.Provider value={authContext}>
      <ProfileProvider>
        <DefaultActionsDelegator />
        <main className={classNames.layout}>
          <Nav className={classNames.nav} />
          {children}
        </main>
      </ProfileProvider>
    </AuthContext.Provider>
  )
}
