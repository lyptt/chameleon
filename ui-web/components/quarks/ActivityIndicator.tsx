import cx from 'classnames'
import { HTMLAttributes } from 'react'

export default function ActivityIndicator({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cx(
        'chameleon-activity-indicator',

        className
      )}
      {...props}
    />
  )
}
