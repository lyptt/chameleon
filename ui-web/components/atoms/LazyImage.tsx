import React, {
  useState,
  useCallback,
  DetailedHTMLProps,
  ImgHTMLAttributes,
} from 'react'
import { useBlurhash } from '@/core/useBlurhash'
import { useInView } from 'react-intersection-observer'
import cx from 'classnames'

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

  const handleOnLoad = useCallback(() => {
    setImgLoaded(true)

    setTimeout(() => {
      setImgFaded(true)
    }, 160)
  }, [])

  return (
    <div
      className={cx('chameleon-image', className, {
        'chameleon-image--loaded': imgLoaded,
        'chameleon-image--faded': imgFaded,
      })}
    >
      <img
        className={cx('chameleon-image__content', contentClassName)}
        ref={ref}
        alt={alt}
        {...rest}
        loading={loading}
        onLoad={handleOnLoad}
      />
      <div
        className="chameleon-image__overlay"
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
