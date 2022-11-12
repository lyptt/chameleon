import '../styles/globals.css'
import type { AppContext, AppProps } from 'next/app'
import App from 'next/app'
import {
  buildAuthContext,
  IAuthContext,
} from '@/components/organisms/AuthContext'
import MainLayout from '@/components/layouts/MainLayout'

interface IChameleonAppProps extends AppProps {
  defaultAuthContext?: IAuthContext
}

export default function ChameleonApp(props: IChameleonAppProps) {
  const { Component, pageProps, defaultAuthContext } = props
  return (
    <MainLayout defaultAuthContext={defaultAuthContext}>
      <Component {...pageProps} />
    </MainLayout>
  )
}

function getDocumentCookie() {
  if (typeof window === 'undefined') {
    return ''
  }

  return window.document.cookie
}

ChameleonApp.getInitialProps = async (context: AppContext) => {
  const ctx = await App.getInitialProps(context)

  const authContext = buildAuthContext(
    context.ctx.req?.headers.cookie || getDocumentCookie()
  )

  if (!!process.env.OAUTH_REDIRECT_URI && typeof global !== 'undefined') {
    ;(global as any).cookie = context.ctx.req?.headers.cookie
  }

  return {
    ...ctx,
    defaultAuthContext: authContext,
  }
}
