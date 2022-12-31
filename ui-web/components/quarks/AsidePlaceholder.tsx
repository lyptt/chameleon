import { HTMLProps } from 'react'
import cx from 'classnames'

export interface AsidePlaceholderProps extends HTMLProps<HTMLDivElement> {
  rows?: number
}

export default function AsidePlaceholder({
  rows,
  className,
  ...rest
}: AsidePlaceholderProps) {
  return (
    <div className={cx('orbit-aside-placeholder', className)} {...rest}>
      {new Array(rows || 3).fill(0).map((_, i) => (
        <div key={i} className="orbit-aside-placeholder__block">
          &nbsp;
        </div>
      ))}
    </div>
  )
}
