import Config from './config'

function buildDefaultHeaders(authToken: string): any {
  return {
    headers: {
      Authorization: `Bearer ${authToken}`,
      Accept: 'application/json',
      'Content-Type': 'application/json',
    },
    mode: 'cors',
    credentials: 'include',
  } as any
}

function buildUnauthenticatedHeaders(): any {
  return {
    headers: {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    },
    mode: 'cors',
  } as any
}

export interface IListResponse<T> {
  data: T[]
  page: number
  total_items: number
  total_pages: number
}

export interface IObjectResponse<T> {
  data: T
}

export interface IRecordResponse {
  id: string
}

export interface INewJobResponse {
  job_id: string
}

export interface IProfile {
  user_id: string
  fediverse_id: string
  handle?: string
  avatar_url?: string
  email?: string
  intro_md?: string
  intro_html?: string
  url_1?: string
  url_2?: string
  url_3?: string
  url_4?: string
  url_5?: string
  url_1_title?: string
  url_2_title?: string
  url_3_title?: string
  url_4_title?: string
  url_5_title?: string
  created_at: string
}

export interface IProfileStats {
  following_count: number
  followers_count: number
  following_user: boolean
  user_is_you: boolean
}

export enum AccessType {
  Unknown = 'unknown',
  Shadow = 'shadow',
  Unlisted = 'unlisted',
  Private = 'private',
  FollowersOnly = 'followers_only',
  PublicLocal = 'public_local',
  PublicFederated = 'public_federated',
}

export interface IPostAttachment {
  attachment_id: string
  user_id: string
  post_id: string
  uri?: string
  width: number
  height: number
  content_type?: string
  blurhash?: string
  created_at: number
}

export interface IPost {
  post_id: string
  user_id: string
  orbit_id?: string
  user_handle: string
  user_fediverse_id: string
  user_avatar_url?: string
  uri: string
  title?: string
  content_md: string
  content_html: string
  visibility: AccessType
  created_at: number
  updated_at: number
  likes: number
  liked?: boolean
  comments: number
  attachments: IPostAttachment[]
  orbit_shortcode?: string
  orbit_name?: string
  orbit_uri?: string
  orbit_avatar_uri?: string
}

export interface INewPost {
  content_md: string
  visibility: AccessType
  orbit_name?: string
  attachment_count: number
}

export enum JobStatus {
  NotStarted = 'not_started',
  InProgress = 'in_progress',
  Done = 'done',
  Failed = 'failed',
}

export interface IJob {
  job_id: string
  record_id?: string
  created_by_id?: string
  created_at: number
  updated_at: number
  status: JobStatus
  failed_count: number
}

export interface IComment {
  comment_id: string
  user_id: string
  post_id: string
  content_md: string
  content_html: string
  created_at: number
  updated_at: number
  user_handle: string
  user_fediverse_id: string
  user_avatar_url?: string
  likes: number
  liked?: boolean
}

export interface IOrbit {
  orbit_id: string
  created_at: string
  updated_at: string
  shortcode: string
  name: string
  description_md: string
  description_html: string
  avatar_uri?: string
  banner_uri?: string
  uri: string
  is_external: boolean
}

export interface IOrbitProfile extends IOrbit {
  joined: boolean
  moderating: boolean
}

export interface INewOrbit {
  name: string
  description_md: string
}

export interface INewProfile {
  handle: string
  intro_md: string
  email?: string
  password?: string
  attachments: File[]
  links: {
    title: string
    url: string
  }[]
}

type IPatchProp = 'erase' | { replace: any }

interface IProfileUpdateRequest {
  handle?: string
  password?: string
  intro_md?: IPatchProp
  email?: IPatchProp
  url_1?: IPatchProp
  url_2?: IPatchProp
  url_3?: IPatchProp
  url_4?: IPatchProp
  url_5?: IPatchProp
  url_1_title?: IPatchProp
  url_2_title?: IPatchProp
  url_3_title?: IPatchProp
  url_4_title?: IPatchProp
  url_5_title?: IPatchProp
}

export async function fetchProfile(authToken: string): Promise<IProfile> {
  const response = await fetch(`${Config.apiUri}/profile`, {
    ...buildDefaultHeaders(authToken),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchUserProfile(
  handle: string,
  authToken: string | undefined
): Promise<IProfile> {
  const response = await fetch(`${Config.apiUri}/users/${handle}`, {
    ...(authToken
      ? buildDefaultHeaders(authToken)
      : buildUnauthenticatedHeaders()),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchUserStats(
  handle: string,
  authToken: string | undefined
): Promise<IObjectResponse<IProfileStats>> {
  const response = await fetch(`${Config.apiUri}/users/${handle}/stats`, {
    ...(authToken
      ? buildDefaultHeaders(authToken)
      : buildUnauthenticatedHeaders()),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchFederatedFeed(
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IPost>> {
  const response = await fetch(
    `${Config.apiUri}/feed/federated?page=${page}&page_size=${pageSize}`,
    {
      ...buildUnauthenticatedHeaders(),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchOwnFeed(
  authToken: string,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IPost>> {
  const response = await fetch(
    `${Config.apiUri}/feed?page=${page}&page_size=${pageSize}`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchOwnFriendsFeed(
  authToken: string,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IPost>> {
  const response = await fetch(
    `${Config.apiUri}/feed/friends?page=${page}&page_size=${pageSize}`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchUserFeed(
  handle: string,
  authToken: string | undefined,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IPost>> {
  const response = await fetch(
    `${Config.apiUri}/users/${handle}/feed?page=${page}&page_size=${pageSize}`,
    {
      ...(authToken
        ? buildDefaultHeaders(authToken)
        : buildUnauthenticatedHeaders()),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchOrbitFeed(
  shortcode: string,
  authToken: string | undefined,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IPost>> {
  const response = await fetch(
    `${Config.apiUri}/orbits/${shortcode}/feed?page=${page}&page_size=${pageSize}`,
    {
      ...(authToken
        ? buildDefaultHeaders(authToken)
        : buildUnauthenticatedHeaders()),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchUserOrbits(
  handle: string,
  authToken: string | undefined,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IOrbit>> {
  const response = await fetch(
    `${Config.apiUri}/users/${handle}/orbits?page=${page}&page_size=${pageSize}`,
    {
      ...(authToken
        ? buildDefaultHeaders(authToken)
        : buildUnauthenticatedHeaders()),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchOrbit(
  shortcode: string,
  authToken: string | undefined
): Promise<IObjectResponse<IOrbitProfile>> {
  const response = await fetch(`${Config.apiUri}/orbits/${shortcode}`, {
    ...(authToken
      ? buildDefaultHeaders(authToken)
      : buildUnauthenticatedHeaders()),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchOrbitById(
  id: string,
  authToken: string | undefined
): Promise<IObjectResponse<IOrbitProfile>> {
  const response = await fetch(`${Config.apiUri}/orbit/${id}`, {
    ...(authToken
      ? buildDefaultHeaders(authToken)
      : buildUnauthenticatedHeaders()),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function submitPost(
  post: INewPost,
  authToken: string
): Promise<IRecordResponse | INewJobResponse> {
  const response = await fetch(`${Config.apiUri}/feed`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify(post),
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export function submitPostImage(
  postId: string,
  files: File[],
  authToken: string,
  onProgress?: (progress: number) => void
): Promise<INewJobResponse> {
  const form = new FormData()
  files.forEach((file) => form.append('images[]', file))

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.upload.addEventListener('progress', (e) =>
      onProgress?.(e.loaded / e.total)
    )
    xhr.addEventListener('load', () => {
      if (xhr.status !== 200) {
        return reject(new Error('Request failed'))
      }

      try {
        resolve(JSON.parse(xhr.responseText))
      } catch (err) {
        reject(err)
      }
    })
    xhr.addEventListener('error', () => reject(new Error('File upload failed')))
    xhr.addEventListener('abort', () =>
      reject(new Error('File upload aborted'))
    )
    xhr.open('POST', `${Config.apiUri}/feed/${postId}`, true)
    xhr.setRequestHeader('Authorization', `Bearer ${authToken}`)
    xhr.setRequestHeader('Accept', `application/json`)
    xhr.send(form)
  })
}

export async function getJobStatus(
  jobId: string,
  authToken: string
): Promise<IJob> {
  const response = await fetch(`${Config.apiUri}/job/${jobId}`, {
    ...buildDefaultHeaders(authToken),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchPost(
  postId: string,
  authToken?: string
): Promise<IObjectResponse<IPost>> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}`, {
    ...(authToken
      ? buildDefaultHeaders(authToken)
      : buildUnauthenticatedHeaders()),
    method: 'GET',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function fetchPostComments(
  postId: string,
  authToken: string | undefined,
  page: number,
  pageSize: number = 20
): Promise<IListResponse<IComment>> {
  const response = await fetch(
    `${Config.apiUri}/feed/${postId}/comments?page=${page}&page_size=${pageSize}`,
    {
      ...(authToken
        ? buildDefaultHeaders(authToken)
        : buildUnauthenticatedHeaders()),
      method: 'GET',
    }
  )

  if (response.status !== 200) {
    if (response.status === 404) {
      return { data: [], page, total_items: 0, total_pages: 0 }
    }
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function likePost(
  postId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}/likes`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
  })

  if (response.status !== 201) {
    throw new Error('Request failed')
  }
}

export async function unlikePost(
  postId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}/likes`, {
    ...buildDefaultHeaders(authToken),
    method: 'DELETE',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function createPostComment(
  content: string,
  postId: string,
  authToken: string
): Promise<IObjectResponse<IComment>> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}/comments`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify({
      content_md: content,
    }),
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export async function deletePostComment(
  postId: string,
  commentId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(
    `${Config.apiUri}/feed/${postId}/comments/${commentId}`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'DELETE',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function createPostCommentLike(
  postId: string,
  commentId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(
    `${Config.apiUri}/feed/${postId}/comments/${commentId}/likes`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'POST',
    }
  )

  if (response.status !== 201) {
    throw new Error('Request failed')
  }
}

export async function deletePostCommentLike(
  postId: string,
  commentId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(
    `${Config.apiUri}/feed/${postId}/comments/${commentId}/likes`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'DELETE',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function createFollow(
  userHandle: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/users/${userHandle}/follows`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function deleteFollow(
  userHandle: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/users/${userHandle}/follows`, {
    ...buildDefaultHeaders(authToken),
    method: 'DELETE',
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function createComment(
  postId: string,
  commentMd: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}/comments`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify({
      comment_md: commentMd,
    }),
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function deleteComment(
  postId: string,
  commentId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(
    `${Config.apiUri}/feed/${postId}/comments/${commentId}`,
    {
      ...buildDefaultHeaders(authToken),
      method: 'POST',
    }
  )

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export async function submitOrbit(
  orbit: INewOrbit,
  authToken: string
): Promise<IRecordResponse> {
  const response = await fetch(`${Config.apiUri}/orbits`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify(orbit),
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }

  return await response.json()
}

export function submitOrbitImage(
  orbitId: string,
  files: File[],
  authToken: string,
  onProgress?: (progress: number) => void
): Promise<void> {
  const form = new FormData()
  files.forEach((file) => form.append('images[]', file))

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.upload.addEventListener('progress', (e) =>
      onProgress?.(e.loaded / e.total)
    )
    xhr.addEventListener('load', () => {
      if (xhr.status !== 200) {
        return reject(new Error('Request failed'))
      }

      try {
        resolve()
      } catch (err) {
        reject(err)
      }
    })
    xhr.addEventListener('error', () => reject(new Error('File upload failed')))
    xhr.addEventListener('abort', () =>
      reject(new Error('File upload aborted'))
    )
    xhr.open('POST', `${Config.apiUri}/orbit/${orbitId}/assets`, true)
    xhr.setRequestHeader('Authorization', `Bearer ${authToken}`)
    xhr.setRequestHeader('Accept', `application/json`)
    xhr.send(form)
  })
}

export async function submitProfile(
  currentProfile: IProfile,
  profile: INewProfile,
  authToken: string
): Promise<void> {
  const req: IProfileUpdateRequest = {}

  if (!!profile.handle && profile.handle !== currentProfile.handle) {
    req.handle = profile.handle
  }

  if (!!profile.password) {
    req.password = profile.password
  }

  if (currentProfile.intro_md !== profile.intro_md) {
    req.intro_md = profile.intro_md ? { replace: profile.intro_md } : 'erase'
  }
  if (currentProfile.email !== profile.email) {
    req.email = profile.email ? { replace: profile.email } : 'erase'
  }
  if (currentProfile.url_1 !== profile.links[0]?.url) {
    req.url_1 = profile.links[0]?.url
      ? { replace: profile.links[0]?.url }
      : 'erase'
  }
  if (currentProfile.url_2 !== profile.links[1]?.url) {
    req.url_2 = profile.links[1]?.url
      ? { replace: profile.links[1]?.url }
      : 'erase'
  }
  if (currentProfile.url_3 !== profile.links[2]?.url) {
    req.url_3 = profile.links[2]?.url
      ? { replace: profile.links[2]?.url }
      : 'erase'
  }
  if (currentProfile.url_4 !== profile.links[3]?.url) {
    req.url_4 = profile.links[3]?.url
      ? { replace: profile.links[3]?.url }
      : 'erase'
  }
  if (currentProfile.url_5 !== profile.links[4]?.url) {
    req.url_5 = profile.links[4]?.url
      ? { replace: profile.links[4]?.url }
      : 'erase'
  }
  if (currentProfile.url_1_title !== profile.links[0]?.title) {
    req.url_1_title = profile.links[0]?.title
      ? { replace: profile.links[0]?.title }
      : 'erase'
  }
  if (currentProfile.url_2_title !== profile.links[1]?.title) {
    req.url_2_title = profile.links[1]?.title
      ? { replace: profile.links[1]?.title }
      : 'erase'
  }
  if (currentProfile.url_3_title !== profile.links[2]?.title) {
    req.url_3_title = profile.links[2]?.title
      ? { replace: profile.links[2]?.title }
      : 'erase'
  }
  if (currentProfile.url_4_title !== profile.links[3]?.title) {
    req.url_4_title = profile.links[3]?.title
      ? { replace: profile.links[3]?.title }
      : 'erase'
  }
  if (currentProfile.url_5_title !== profile.links[4]?.title) {
    req.url_5_title = profile.links[4]?.title
      ? { replace: profile.links[4]?.title }
      : 'erase'
  }

  const response = await fetch(`${Config.apiUri}/profile`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify(req),
  })

  if (response.status !== 200) {
    throw new Error('Request failed')
  }
}

export function submitProfileImage(
  files: File[],
  authToken: string,
  onProgress?: (progress: number) => void
): Promise<void> {
  const form = new FormData()
  files.forEach((file) => form.append('images[]', file))

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest()
    xhr.upload.addEventListener('progress', (e) =>
      onProgress?.(e.loaded / e.total)
    )
    xhr.addEventListener('load', () => {
      if (xhr.status !== 200) {
        return reject(new Error('Request failed'))
      }

      try {
        resolve()
      } catch (err) {
        reject(err)
      }
    })
    xhr.addEventListener('error', () => reject(new Error('File upload failed')))
    xhr.addEventListener('abort', () =>
      reject(new Error('File upload aborted'))
    )
    xhr.open('POST', `${Config.apiUri}/profile/assets`, true)
    xhr.setRequestHeader('Authorization', `Bearer ${authToken}`)
    xhr.setRequestHeader('Accept', `application/json`)
    xhr.send(form)
  })
}

export async function joinOrbit(
  orbitId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/orbit/${orbitId}/join`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
  })

  if (response.status !== 201) {
    throw new Error('Request failed')
  }
}

export async function leaveOrbit(
  orbitId: string,
  authToken: string
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/orbit/${orbitId}/leave`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
  })

  if (response.status !== 201) {
    throw new Error('Request failed')
  }
}
