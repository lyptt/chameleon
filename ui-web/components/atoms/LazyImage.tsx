import React, { useState, useCallback } from 'react'
import { useBlurhash } from '@/core/useBlurhash'
import { useInView } from 'react-intersection-observer'
import cx from 'classnames'
import classNames from './LazyImage.module.css'

type Props = React.DetailedHTMLProps<
  React.ImgHTMLAttributes<HTMLImageElement>,
  HTMLImageElement
> & { blurhash?: string | null }

export function LazyImage(props: Props) {
  const { loading = 'lazy', blurhash, style, className, ...rest } = props

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
      className={cx(classNames.container, className, {
        [classNames.loaded]: imgLoaded,
        [classNames.faded]: imgFaded,
      })}
    >
      <img ref={ref} {...rest} loading={loading} onLoad={handleOnLoad} />
      <div
        className={classNames.overlay}
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
