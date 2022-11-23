import cx from 'classnames'
import { useEffect, useRef, useState } from 'react'
import Button from '@/components/quarks/Button'
import PlainButton from '@/components/quarks/PlainButton'
import IconButton, { IconButtonIcon } from '../atoms/IconButton'
import { IComment, IPost } from '@/core/api'

export interface INewCommentFormProps {
  post?: IPost
  comment?: IComment
  className?: string
  onCancel?: () => void
  onSubmit?: (comment: string, visibility: string) => void
}

export default function NewCommentForm({
  className,
  comment: originalComment,
  post: originalPost,
  onCancel,
  onSubmit,
}: INewCommentFormProps) {
  const [visibility, setVisibility] = useState<string | undefined>(
    'public_federated'
  )
  const [comment, setComment] = useState<string | undefined>()

  const handleSubmit = () => {
    onSubmit?.(comment || '', visibility || 'public_federated')
  }

  const textFieldRef = useRef<HTMLTextAreaElement>(null)

  useEffect(() => {
    if (originalComment) {
      setComment(`@${originalComment.user_fediverse_id} `)
      textFieldRef.current?.focus()
    } else if (originalPost) {
      setComment(`@${originalPost.user_fediverse_id} `)
      textFieldRef.current?.focus()
    } else {
      setComment('')
    }
    setVisibility('public_federated')
  }, [originalComment, originalPost])

  return (
    <>
      <form
        className={cx('chameleon-form-new-comment', className)}
        onSubmit={(e) => e.preventDefault()}
      >
        <div className="chameleon-form-new-comment__preview-content">
          <textarea
            className="chameleon-form-new-comment__caption-field"
            placeholder="Write a caption..."
            value={comment}
            disabled={!originalComment && !originalPost}
            onChange={(e) => setComment(e.target.value)}
            ref={textFieldRef}
          />
        </div>
        <div className="chameleon-form-new-comment__preview-options">
          <IconButton
            className="chameleon-form-new-comment__visibility-button"
            contentClassName="chameleon-form-new-comment__visibility-button-icon"
            icon={IconButtonIcon.VisibilityFederated}
          />
          <span
            className={cx('chameleon-form-new-comment__character-count', {
              'chameleon-form-new-comment__character-count--empty':
                !!comment && comment.length > 500,
            })}
          >
            {500 - (comment?.length || 0)}
          </span>
        </div>
      </form>

      <div className="chameleon-form-new-comment__button-stack">
        <PlainButton
          brand
          className="chameleon-form-new-comment__left-action"
          onClick={onCancel}
          disabled={!originalComment && !originalPost}
        >
          Cancel
        </PlainButton>
        <Button
          bold
          className="chameleon-form-new-comment__right-action"
          onClick={handleSubmit}
          disabled={
            (!!comment && comment.length > 500) ||
            (!originalComment && !originalPost)
          }
        >
          {visibility === 'public_local' || visibility === 'public_federated'
            ? 'Publish!'
            : 'Publish'}
        </Button>
      </div>
    </>
  )
}
