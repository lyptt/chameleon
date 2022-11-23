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

export interface ILazyImageProps
  extends DetailedHTMLProps<
    ImgHTMLAttributes<HTMLImageElement>,
    HTMLImageElement
  > {
  blurhash?: string | null
  contentClassName?: string
}

export function LazyImage({
  loading = 'lazy',
  blurhash,
  style,
  className,
  contentClassName,
  alt,
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

  return (
    <div className={cx('chameleon-image', className)}>
      <img
        className={cx('chameleon-image__content', contentClassName)}
        ref={ref}
        alt={alt}
        {...rest}
        loading={loading}
        onLoad={handleOnLoad}
      />
      <div
        className={cx('chameleon-image__overlay', {
          'chameleon-image__overlay--loaded': imgLoaded,
          'chameleon-image__overlay--faded': imgFaded,
        })}
        style={{
          backgroundImage: `url("${blurUrl}")`,
          backgroundSize:
            rest.width && rest.height
              ? `${rest.width}px ${rest.height}px`
              : '100% 100%',
        }}
      />
    </div>
  )
}
