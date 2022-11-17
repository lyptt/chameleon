import classNames from './Progress.module.css'
import cx from 'classnames'
import { ProgressHTMLAttributes } from 'react'

export default function Progress({
  className,
  ...props
}: ProgressHTMLAttributes<HTMLProgressElement>) {
  return <progress className={cx(className, classNames.progress)} {...props} />
}
