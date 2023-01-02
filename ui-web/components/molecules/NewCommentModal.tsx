import { useCallback } from 'react'
import ReactModal from 'react-modal'
import CreateLayout, {
  CreateForm,
  CreateFormButtons,
  CreateFormGroup,
} from '@/components/layouts/CreateLayout'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  feedActionAddPostComment,
  useFeed,
} from '@/components/organisms/FeedContext'
import { useTheme } from '@/components/organisms/ThemeContext'
import { Formik } from 'formik'
import cx from 'classnames'
import {
  postActionAddPostComment,
  postActionDeselect,
  usePost,
} from '../organisms/PostContext'
import { useEffect } from 'react'

export interface NewCommentModalProps {
  postId: string
  open: boolean
  onClose: () => void
}

interface INewCommentForm {
  content_md: string
}

export default function NewCommentModal({
  postId,
  open,
  onClose,
}: NewCommentModalProps) {
  const { session } = useAuth()
  const { state, dispatch } = usePost()
  const { theme } = useTheme()
  const { selectedComment } = state

  const onSubmit: (values: INewCommentForm) => Promise<void> = useCallback(
    async (values) => {
      postActionAddPostComment(
        values.content_md,
        postId,
        session?.access_token,
        dispatch
      )
    },
    [session, onClose]
  )

  useEffect(() => {
    if (selectedComment) {
      postActionDeselect(dispatch)
      onClose()
    }
  }, [selectedComment])

  return (
    <ReactModal
      isOpen={open}
      onRequestClose={() => onClose()}
      portalClassName={theme}
      className={cx('orbit-modal', theme, { 'orbit-modal--visible': open })}
      overlayClassName={cx('orbit-modal-overlay', theme, {
        'orbit-modal-overlay--visible': open,
      })}
    >
      <CreateLayout embedded>
        <Formik
          initialValues={{
            content_md: '',
          }}
          onSubmit={onSubmit}
        >
          <CreateForm title="New Comment">
            <CreateFormGroup
              title="Comment"
              id="content_md"
              name="content_md"
              placeholder="**Hello**, world!"
              as="textarea"
            />
            <CreateFormButtons
              submitTitle="Submit Comment"
              cancelTitle="Cancel"
              onCancel={onClose}
            />
          </CreateForm>
        </Formik>
      </CreateLayout>
    </ReactModal>
  )
}
