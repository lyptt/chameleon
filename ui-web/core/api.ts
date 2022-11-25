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

export interface IPost {
  post_id: string
  user_id: string
  user_handle: string
  user_fediverse_id: string
  user_avatar_url?: string
  uri: string
  content_md: string
  content_html: string
  content_image_uri_small?: string
  content_image_uri_medium?: string
  content_image_uri_large?: string
  content_width_small?: number
  content_width_medium?: number
  content_width_large?: number
  content_height_small?: number
  content_height_medium?: number
  content_height_large?: number
  content_type_small?: string
  content_type_medium?: string
  content_type_large?: string
  content_blurhash?: string
  visibility: AccessType
  created_at: number
  updated_at: number
  likes: number
  liked?: boolean
  comments: number
}

export interface INewPost {
  content_md: string
  visibility: AccessType
}

export enum JobStatus {
  NotStarted = 'not_started',
  InProgress = 'in_progress',
  Done = 'done',
  Failed = 'failed',
}

export interface IJob {
  job_id: string
  completion_record_id?: string
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

export async function submitPost(
  post: INewPost,
  authToken: string
): Promise<IRecordResponse> {
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
  file: File,
  authToken: string,
  onProgress?: (progress: number) => void
): Promise<INewJobResponse> {
  const form = new FormData()
  form.append('images[]', file)

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
): Promise<void> {
  const response = await fetch(`${Config.apiUri}/feed/${postId}/comments`, {
    ...buildDefaultHeaders(authToken),
    method: 'POST',
    body: JSON.stringify({
      content_md: content,
    }),
  })

  if (response.status !== 201) {
    throw new Error('Request failed')
  }
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
