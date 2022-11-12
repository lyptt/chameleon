import {
  profileActionLoadProfile,
  useProfile,
} from '@/components/organisms/ProfileContext'
import { useEffect } from 'react'
import { useAuth } from './AuthContext'

export default function DefaultActionsDelegator() {
  const { session } = useAuth()
  const { state, dispatch } = useProfile()
  useEffect(() => {
    if (!session || state.profile || state.loading || state.loadingFailed) {
      return
    }

    profileActionLoadProfile(session.access_token, dispatch)
  }, [session, state, dispatch])

  return <></>
}
