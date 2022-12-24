import { IPost } from '@/core/api'
import Config from '@/core/config'

export const isUrlAbsolute = (url: string) =>
  url.indexOf('//') === 0
    ? true
    : url.indexOf('://') === -1
    ? false
    : url.indexOf('.') === -1
    ? false
    : url.indexOf('/') === -1
    ? false
    : url.indexOf(':') > url.indexOf('/')
    ? false
    : url.indexOf('://') < url.indexOf('.')
    ? true
    : false

export function buildSrcSet(post: IPost): string {
  const ret = []

  if (post.content_image_uri_large) {
    ret.push(
      isUrlAbsolute(post.content_image_uri_large)
        ? `${post.content_image_uri_large} ${post.content_width_large}w`
        : `${Config.cdn}/${post.content_image_uri_large} ${post.content_width_large}w`
    )
  }

  if (post.content_image_uri_medium) {
    ret.push(
      isUrlAbsolute(post.content_image_uri_medium)
        ? `${post.content_image_uri_medium} ${post.content_width_medium}w`
        : `${Config.cdn}/${post.content_image_uri_medium} ${post.content_width_medium}w`
    )
  }

  if (post.content_image_uri_small) {
    ret.push(
      isUrlAbsolute(post.content_image_uri_small)
        ? `${post.content_image_uri_small} ${post.content_width_small}w`
        : `${Config.cdn}/${post.content_image_uri_small} ${post.content_width_small}w`
    )
  }

  return ret.join(', ')
}

export function determineFallbackContentImageUri(
  post: IPost
): string | undefined {
  if (post.content_image_uri_large) {
    return isUrlAbsolute(post.content_image_uri_large)
      ? post.content_image_uri_large
      : `${Config.cdn}/${post.content_image_uri_large}`
  }

  if (post.content_image_uri_medium) {
    return isUrlAbsolute(post.content_image_uri_medium)
      ? post.content_image_uri_medium
      : `${Config.cdn}/${post.content_image_uri_medium}`
  }

  if (post.content_image_uri_small) {
    return isUrlAbsolute(post.content_image_uri_small)
      ? post.content_image_uri_small
      : `${Config.cdn}/${post.content_image_uri_small}`
  }

  return undefined
}
