import {
  profileActionLoadProfile,
  useProfile,
} from '@/components/organisms/ProfileContext'
import { useEffect } from 'react'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  feedActionLoadFeed,
  FeedType,
  useFeed,
} from '@/components/organisms/FeedContext'
import { useRouter } from 'next/router'
import { postActionDismissPost, usePost } from './PostContext'

export default function DefaultActionsDelegator() {
  const { session } = useAuth()
  const { state: profileState, dispatch: profileDispatch } = useProfile()
  const { state: feedState, dispatch: feedDispatch } = useFeed()
  const { state: postState, dispatch: postDispatch } = usePost()
  const { route, query } = useRouter()

  useEffect(() => {
    if (
      !session ||
      profileState.profile ||
      profileState.loading ||
      profileState.loadingFailed
    ) {
      return
    }

    profileActionLoadProfile(session.access_token, profileDispatch)
  }, [session, profileState, profileDispatch])

  useEffect(() => {
    if (feedState.initialLoadComplete) {
      if (feedState.type === FeedType.PublicFederated && !session) {
        return
      } else if (feedState.type === FeedType.Own && session) {
        return
      }
    }

    if (feedState.loading || feedState.loadingFailed) {
      return
    }

    feedActionLoadFeed(0, session?.access_token, feedDispatch)
  }, [session, feedState, feedDispatch])

  useEffect(() => {
    if (route !== '/users/[userId]/[postId]' && !!postState.post) {
      postActionDismissPost(postDispatch)
    }
  }, [postState, postDispatch, route])

  return <></>
}
