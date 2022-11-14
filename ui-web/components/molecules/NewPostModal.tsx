import classNames from './NewPostModal.module.css'
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
  onOpen?: () => void
  onClose?: () => void
}

export default function NewPostModal({
  className,
  open,
  onClose,
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
    onClose?.()
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
      className={cx(classNames.modal, className, {
        [classNames.opened]: opened,
        [classNames.showingOptions]: !selectingFiles,
      })}
      overlayClassName={classNames.overlay}
      contentLabel="Example Modal"
    >
      <ScrollLock>
        <div className={classNames.content}>
          <div className={classNames.titleBar}>
            {!selectingFiles && (
              <PlainButton
                className={classNames.leftAction}
                onClick={handleBack}
              >
                <IoArrowBack />
              </PlainButton>
            )}
            <span>Create new post</span>
            {!selectingFiles && (
              <PlainButton brand className={classNames.rightAction}>
                Share
              </PlainButton>
            )}
          </div>
          {selectingFiles && (
            <Dropzone onDrop={onDrop}>
              {({ getRootProps, getInputProps }) => (
                <div className={classNames.upload} {...getRootProps()}>
                  <div className={classNames.uploadFeature}>
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
                className={classNames.preview}
                ref={previewRef}
                draggable={false}
              />
              <div className={classNames.previewOptions}>
                <div className={classNames.profileBar}>
                  <img
                    src={
                      profileState.profile?.avatar_url ?? transparentPixelUri
                    }
                    alt={profileState.profile?.handle ?? ''}
                  />
                  <div className={classNames.name}>
                    {profileState.profile?.handle ?? ''}
                  </div>
                </div>
                <textarea
                  className={classNames.captionField}
                  placeholder="Write a caption..."
                  value={caption}
                  onChange={(e) => setCaption(e.target.value)}
                />
                <div className={classNames.visibility}>
                  <div
                    className={cx(classNames.radio, {
                      [classNames.active]: visibility === 'public_federated',
                    })}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      id="public_federated"
                      value="public_federated"
                      checked={visibility === 'public_federated'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={classNames.icon}>
                      <IoEarth />
                    </div>
                    <label htmlFor="public_federated">Public</label>
                  </div>
                  <div
                    className={cx(classNames.radio, {
                      [classNames.active]: visibility === 'public_local',
                    })}
                  >
                    <input
                      id="public_local"
                      type="radio"
                      name="visibility"
                      value="public_local"
                      checked={visibility === 'public_local'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={classNames.icon}>
                      <IoCafeOutline />
                    </div>
                    <label htmlFor="public_local">Local</label>
                  </div>
                  <div
                    className={cx(classNames.radio, {
                      [classNames.active]: visibility === 'followers_only',
                    })}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      value="followers_only"
                      id="followers_only"
                      checked={visibility === 'followers_only'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={classNames.icon}>
                      <IoPeopleOutline />
                    </div>
                    <label htmlFor="followers_only">Followers</label>
                  </div>
                  <div
                    className={cx(classNames.radio, {
                      [classNames.active]: visibility === 'unlisted',
                    })}
                  >
                    <input
                      type="radio"
                      name="visibility"
                      value="unlisted"
                      id="unlisted"
                      checked={visibility === 'unlisted'}
                      onChange={onVisibilityChanged}
                    />
                    <div className={classNames.icon}>
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
