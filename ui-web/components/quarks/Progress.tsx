import cx from 'classnames'
import { HTMLProps, ProgressHTMLAttributes } from 'react'

export interface ProgressProps extends HTMLProps<HTMLDivElement> {
  value?: number
  max: number
}

export default function Progress({
  className,
  value,
  max,
  ...props
}: ProgressProps) {
  return (
    <div
      className={cx('orbit-progress', className, {
        'orbit-progress--indeterminate': value === undefined,
      })}
      {...props}
    >
      <div
        className="orbit-progress__value"
        style={{ flex: !!value ? `${value / max}` : '1' }}
        aria-hidden="true"
      />
      <progress className="orbit-progress__semantic" value={value} max={max} />
    </div>
  )
}
