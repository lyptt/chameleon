import React, {
  useState,
  useCallback,
  DetailedHTMLProps,
  ImgHTMLAttributes,
} from 'react'
import { useBlurhash } from '@/core/useBlurhash'
import { useInView } from 'react-intersection-observer'
import cx from 'classnames'
import dayjs from 'dayjs'
import Link from 'next/link'

export interface ILazyImageProps
  extends DetailedHTMLProps<
    ImgHTMLAttributes<HTMLImageElement>,
    HTMLImageElement
  > {
  blurhash?: string | null
  contentClassName?: string
  thumbnailSrc?: string
  asLink?: boolean
}

export function LazyImage({
  loading = 'lazy',
  blurhash,
  style,
  className,
  contentClassName,
  alt,
  src,
  thumbnailSrc,
  asLink,
  ...rest
}: ILazyImageProps) {
  const [imgLoaded, setImgLoaded] = useState(false)
  const [imgFaded, setImgFaded] = useState(false)
  const [ref, inView] = useInView({ rootMargin: '110%' })
  const blurUrl = useBlurhash(inView && !imgFaded ? blurhash : null)
  const [initialLoadDate] = useState(new Date())

  const handleOnLoad = useCallback(() => {
    setImgLoaded(true)
    if (dayjs().isAfter(dayjs(initialLoadDate).add(1, 'second'))) {
      setTimeout(() => {
        setImgFaded(true)
      }, 160)
    } else {
      setImgFaded(true)
    }
  }, [initialLoadDate])

  const content = () => (
    <>
      <img
        className={cx('orbit-image__content', contentClassName)}
        ref={ref}
        alt={alt}
        src={thumbnailSrc}
        {...rest}
        loading={loading}
        onLoad={handleOnLoad}
      />
      <div
        className={cx('orbit-image__overlay', {
          'orbit-image__overlay--loaded': imgLoaded,
          'orbit-image__overlay--faded': imgFaded,
        })}
        style={{
          backgroundImage: `url("${blurUrl}")`,
          backgroundSize:
            rest.width && rest.height
              ? `${rest.width}px ${rest.height}px`
              : '100% 100%',
        }}
      />
    </>
  )

  if (asLink) {
    return (
      <Link
        className={cx('orbit-image', className)}
        href={src || 'about:blank'}
        target="blank"
      >
        {content()}
      </Link>
    )
  }

  return <div className={cx('orbit-image', className)}>{content()}</div>
}
