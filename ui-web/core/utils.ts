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

export const cdnUrl = (url: string) => {
  if (url.startsWith('data:')) {
    return url
  }

  if (!isUrlAbsolute(url)) {
    return `${Config.cdn}${url}`
  } else {
    return url
  }
}
