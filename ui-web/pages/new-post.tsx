import CreateLayout from '@/components/layouts/CreateLayout'
import { HTMLAttributes } from 'react'
import cx from 'classnames'

export default function NewPostPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <CreateLayout
      className={cx('orbit-page-new-post', className)}
    ></CreateLayout>
  )
}
