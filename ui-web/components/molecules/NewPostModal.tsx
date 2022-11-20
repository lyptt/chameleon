import cx from 'classnames'
import Modal from 'react-modal'
import ScrollLock from 'react-scrolllock'
import { ChangeEvent, useCallback, useEffect, useState } from 'react'
import {
  IoArrowBack,
  IoCafeOutline,
  IoEarth,
  IoImagesOutline,
  IoLockOpen,
  IoPeopleOutline,
} from 'react-icons/io5'
import Button from '../quarks/Button'
import Dropzone from 'react-dropzone'
import PlainButton from '../quarks/PlainButton'
import { useProfile } from '../organisms/ProfileContext'

Modal.setAppElement('#__next')

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export interface INewPostModal {
  className?: string
  open: boolean
  onCancel?: () => void
  onSubmit?: (visibility: string, file: File, contentMd: string) => void
}

export default function NewPostModal({
  className,
  open,
  onCancel,
  onSubmit,
}: INewPostModal) {
  const { state: profileState } = useProfile()
  const [opened, setOpened] = useState(false)
  const [selectingFiles, setSelectingFiles] = useState(true)
  const [selectedFiles, setSelectedFiles] = useState<File[]>([])
  const [visibility, setVisibility] = useState<string | undefined>(
    'public_federated'
  )
  const [caption, setCaption] = useState<string | undefined>()
  const [previewEl, setPreviewEl] = useState<HTMLImageElement | null>()
  const previewRef = useCallback(
    ($el: HTMLImageElement | null) => {
      setPreviewEl($el)

      if (!selectedFiles || !selectedFiles.length || !previewEl) {
        return
      }

      var fr = new FileReader()
      fr.onload = () => {
        if (!fr.result) {
          return
        }

        previewEl.src = fr.result as any
      }

      fr.readAsDataURL(selectedFiles[0])
    },
    [previewEl, setPreviewEl, selectedFiles]
  )

  useEffect(() => {
    if (!open) {
      return
    }

    setOpened(true)
  }, [open])

  const handleClose = () => {
    onCancel?.()
    setTimeout(() => {
      setOpened(false)
      setSelectingFiles(true)
    }, 50)
  }

  const handleSubmit = () => {
    onSubmit?.(
      visibility || 'public_federated',
      selectedFiles[0],
      caption || ''
    )
    setTimeout(() => {
      setOpened(false)
      setSelectingFiles(true)
    }, 50)
  }

  const handleBack = () => {
    setSelectingFiles(true)
    setSelectedFiles([])
    setCaption(undefined)
    setVisibility('public_federated')
  }

  const onDrop = (acceptedFiles: any) => {
    setSelectingFiles(false)
    setSelectedFiles(acceptedFiles)
  }

  const onVisibilityChanged = (e: ChangeEvent<HTMLInputElement>) => {
    setVisibility(e.target.value)
  }

  return (
    <Modal
      isOpen={open}
      onRequestClose={handleClose}
      className={cx('chameleon-modal-new-post', className, {
        'chameleon-modal-new-post--opened': opened,
        'chameleon-modal-new-post--showing-options': !selectingFiles,
      })}
      overlayClassName="chameleon-modal-new-post__overlay"
      contentLabel="New Post"
    >
      <ScrollLock>
        <div className={cx('chameleon-modal-new-post__content')}>
          <div className={cx('chameleon-modal-new-post__title-bar')}>
            {!selectingFiles && (
              <PlainButton
                className={cx('chameleon-modal-new-post__left-action')}
                onClick={handleBack}
              >
                <IoArrowBack />
              </PlainButton>
            )}
            <span>Create new post</span>
            {!selectingFiles && (
              <PlainButton
                brand
                className={cx('chameleon-modal-new-post__right-action')}
                onClick={handleSubmit}
              >
                Share
              </PlainButton>
            )}
          </div>
          {selectingFiles && (
            <Dropzone onDrop={onDrop}>
              {({ getRootProps, getInputProps }) => (
                <div
                  className={cx('chameleon-modal-new-post__upload')}
                  {...getRootProps()}
                >
                  <div
                    className={cx('chameleon-modal-new-post__upload-feature')}
                  >
                    <IoImagesOutline />
                    <p>Drag your photos here</p>
                    <Button>Select from computer</Button>
                  </div>
                  <form>
                    <input
                      type="file"
                      name="images"
                      {...getInputProps()}
                      accept="image/png,image/jpeg,image/avif,image/webp"
                    />
                  </form>
                </div>
              )}
            </Dropzone>
          )}
          {!selectingFiles && selectedFiles.length > 0 && (
            <>
              <img
                className={cx('chameleon-modal-new-post__preview')}
                alt="Preview of your new photo"
                ref={previewRef}
                draggable={false}
              />
              <div className={cx('chameleon-modal-new-post__preview-options')}>
                <div className={cx('chameleon-modal-new-post__profile-bar')}>
                  <img
                    src={
                      profileState.profile?.avatar_url ?? transparentPixelUri
                    }
                    alt={profileState.profile?.handle ?? ''}
                  />
                  <div className={cx('chameleon-modal-new-post__name')}>
                    {profileState.profile?.handle ?? ''}
                  </div>
                </div>
                <textarea
                  className={cx('chameleon-modal-new-post__caption-field')}
                  placeholder="Write a caption..."
                  value={caption}
                  onChange={(e) => setCaption(e.target.value)}
                />
                <div className={cx('chameleon-modal-new-post__visibility')}>
                  <div
                    className={cx(
                      'chameleon-modal-new-post__radio',

                      {
                        'chameleon-modal-new-post__radio--active':
                          visibility === 'public_federated',
                      }
                    )}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      id="public_federated"
                      value="public_federated"
                      checked={visibility === 'public_federated'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={cx('chameleon-modal-new-post__icon')}>
                      <IoEarth />
                    </div>
                    <label htmlFor="public_federated">Public</label>
                  </div>
                  <div
                    className={cx(
                      'chameleon-modal-new-post__radio',

                      {
                        'chameleon-modal-new-post__radio--active':
                          visibility === 'public_local',
                      }
                    )}
                  >
                    <input
                      id="public_local"
                      type="radio"
                      name="visibility"
                      value="public_local"
                      checked={visibility === 'public_local'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={cx('chameleon-modal-new-post__icon')}>
                      <IoCafeOutline />
                    </div>
                    <label htmlFor="public_local">Local</label>
                  </div>
                  <div
                    className={cx(
                      'chameleon-modal-new-post__radio',

                      {
                        'chameleon-modal-new-post__radio--active':
                          visibility === 'followers_only',
                      }
                    )}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      value="followers_only"
                      id="followers_only"
                      checked={visibility === 'followers_only'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={cx('chameleon-modal-new-post__icon')}>
                      <IoPeopleOutline />
                    </div>
                    <label htmlFor="followers_only">Followers</label>
                  </div>
                  <div
                    className={cx(
                      'chameleon-modal-new-post__radio',

                      {
                        'chameleon-modal-new-post__radio--active':
                          visibility === 'unlisted',
                      }
                    )}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      value="unlisted"
                      id="unlisted"
                      checked={visibility === 'unlisted'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={cx('chameleon-modal-new-post__icon')}>
                      <IoLockOpen />
                    </div>
                    <label htmlFor="unlisted">Unlisted</label>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      </ScrollLock>
    </Modal>
  )
}
