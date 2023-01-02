import {
  profileActionLoadProfile,
  useProfile,
} from '@/components/organisms/ProfileContext'
import { useEffect } from 'react'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  feedActionLoadFeed,
  feedActionReset,
  FeedType,
  useFeed,
} from '@/components/organisms/FeedContext'
import { useRouter } from 'next/router'
import {
  postActionDismissPost,
  usePost,
} from '@/components/organisms/PostContext'
import {
  orbitActionLoadUserOrbits,
  useOrbits,
} from '@/components/organisms/OrbitContext'
import {
  createActionDismiss,
  useCreate,
} from '@/components/organisms/CreateContext'
import { userActionDismiss, useUser } from './UserContext'

export default function DefaultActionsDelegator() {
  const { session } = useAuth()
  const { state: profileState, dispatch: profileDispatch } = useProfile()
  const { state: feedState, dispatch: feedDispatch } = useFeed()
  const { state: postState, dispatch: postDispatch } = usePost()
  const { state: orbitsState, dispatch: orbitsDispatch } = useOrbits()
  const { state: createState, dispatch: createDispatch } = useCreate()
  const { state: userState, dispatch: userDispatch } = useUser()
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
    if (
      !feedState.initialLoadComplete ||
      feedState.loading ||
      feedState.loadingFailed
    ) {
      return
    }

    if (route === '/') {
      if (session && feedState.type !== FeedType.Own) {
        feedActionReset(feedDispatch)
      } else if (!session && feedState.type !== FeedType.PublicFederated) {
        feedActionReset(feedDispatch)
      }
    }

    if (
      route === '/feed/federated' &&
      feedState.type !== FeedType.PublicFederated
    ) {
      feedActionReset(feedDispatch)
    }

    if (route === '/feed/friends' && feedState.type !== FeedType.Friends) {
      feedActionReset(feedDispatch)
    }

    if (
      route === '/orbits/[orbitShortcode]' &&
      feedState.type !== FeedType.Orbit
    ) {
      feedActionReset(feedDispatch)
    }

    if (
      route === '/orbits/[orbitShortcode]' &&
      feedState.type === FeedType.Orbit &&
      feedState.orbit?.shortcode !== query.orbitShortcode
    ) {
      feedActionReset(feedDispatch)
    }
  }, [feedState, feedDispatch, route, query])

  useEffect(() => {
    if (route !== '/') {
      return
    }

    if (feedState.initialLoadComplete) {
      return
    }

    if (feedState.loading || feedState.loadingFailed) {
      return
    }

    feedActionLoadFeed(
      0,
      session?.access_token,
      undefined,
      feedState.type === FeedType.Friends,
      feedDispatch
    )
  }, [session, feedState, feedDispatch, route])

  useEffect(() => {
    if (route !== '/orbits/[orbitShortcode]' || !orbitsState.orbit) {
      return
    }

    if (feedState.initialLoadComplete) {
      return
    }

    if (feedState.loading || feedState.loadingFailed) {
      return
    }

    feedActionLoadFeed(
      0,
      session?.access_token,
      orbitsState.orbit,
      false,
      feedDispatch
    )
  }, [session, feedState, feedDispatch, orbitsState, route])

  useEffect(() => {
    if (route !== '/feed/friends' || !session) {
      return
    }

    if (feedState.initialLoadComplete) {
      return
    }

    if (feedState.loading || feedState.loadingFailed) {
      return
    }

    feedActionLoadFeed(
      0,
      session.access_token,
      orbitsState.orbit,
      true,
      feedDispatch
    )
  }, [session, feedState, feedDispatch, orbitsState, route])

  useEffect(() => {
    if (route !== '/feed/federated') {
      return
    }

    if (feedState.initialLoadComplete) {
      return
    }

    if (feedState.loading || feedState.loadingFailed) {
      return
    }

    feedActionLoadFeed(0, undefined, orbitsState.orbit, false, feedDispatch)
  }, [session, feedState, feedDispatch, orbitsState, route])

  useEffect(() => {
    if (!profileState.profile?.handle || !session || orbitsState.orbits) {
      return
    }

    if (orbitsState.loading || orbitsState.loadingFailed) {
      return
    }

    orbitActionLoadUserOrbits(
      profileState.profile.handle,
      session.access_token,
      orbitsDispatch
    )
  }, [session, profileState, orbitsState, orbitsDispatch])

  useEffect(() => {
    if (
      route.endsWith('/new-post') ||
      route.endsWith('/new-orbit') ||
      route.endsWith('/edit') ||
      !createState.initialized
    ) {
      return
    }

    createActionDismiss(createDispatch)
  }, [createState, createDispatch, route])

  useEffect(() => {
    if (route !== '/feed/[postId]' && !!postState.post) {
      postActionDismissPost(postDispatch)
    }
  }, [postState, postDispatch, route, query])

  useEffect(() => {
    if (
      route !== '/profile' &&
      route !== '/profile/edit' &&
      route !== '/users/[userHandle]' &&
      !!userState.profile &&
      query.userHandle !== userState.profile.handle
    ) {
      userActionDismiss(userDispatch)
    }
  }, [userState, userDispatch, route, query])

  return <></>
}
