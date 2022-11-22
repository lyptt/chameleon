import cx from 'classnames'
import { ChangeEvent, useCallback, useState } from 'react'
import {
  IoArrowBack,
  IoCafeOutline,
  IoEarth,
  IoImagesOutline,
  IoLockOpen,
  IoPeopleOutline,
} from 'react-icons/io5'
import Button from '@/components/quarks/Button'
import Dropzone from 'react-dropzone'
import PlainButton from '@/components/quarks/PlainButton'
import { useProfile } from '@/components/organisms/ProfileContext'
import IconButton, { IconButtonIcon } from '../atoms/IconButton'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export interface INewPostFormProps {
  className?: string
  onSubmit?: (visibility: string, file: File, contentMd: string) => void
}

export default function NewPostForm({
  className,
  onSubmit,
}: INewPostFormProps) {
  const { state: profileState } = useProfile()
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

  const handleSubmit = () => {
    onSubmit?.(
      visibility || 'public_federated',
      selectedFiles[0],
      caption || ''
    )
    setTimeout(() => {
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
    <>
      <form
        className="chameleon-form-new-post"
        onSubmit={(e) => e.preventDefault()}
      >
        {selectingFiles && (
          <Dropzone onDrop={onDrop}>
            {({ getRootProps, getInputProps }) => (
              <div
                className="chameleon-form-new-post__upload"
                {...getRootProps()}
              >
                <div className="chameleon-form-new-post__upload-feature">
                  <p>Drag your photos here</p>
                  <Button>Select from computer</Button>
                </div>

                <input
                  type="file"
                  name="images"
                  {...getInputProps()}
                  accept="image/png,image/jpeg,image/avif,image/webp"
                />
              </div>
            )}
          </Dropzone>
        )}
        {!selectingFiles && selectedFiles.length > 0 && (
          <>
            <img
              className="chameleon-form-new-post__preview"
              alt="Preview of your new photo"
              ref={previewRef}
              draggable={false}
            />
            <div className="chameleon-form-new-post__preview-content">
              <textarea
                className="chameleon-form-new-post__caption-field"
                placeholder="Write a caption..."
                value={caption}
                onChange={(e) => setCaption(e.target.value)}
              />
            </div>
            <div className="chameleon-form-new-post__preview-options">
              <IconButton
                className="chameleon-form-new-post__visibility-button"
                contentClassName="chameleon-form-new-post__visibility-button-icon"
                icon={IconButtonIcon.VisibilityFederated}
              />
              <span
                className={cx('chameleon-form-new-post__character-count', {
                  'chameleon-form-new-post__character-count--empty':
                    !!caption && caption.length > 500,
                })}
              >
                {500 - (caption?.length || 0)}
              </span>
            </div>
          </>
        )}
      </form>

      {!selectingFiles && (
        <div className="chameleon-form-new-post__button-stack">
          <PlainButton
            brand
            className="chameleon-form-new-post__left-action"
            onClick={handleBack}
          >
            Cancel
          </PlainButton>
          <Button
            bold
            className="chameleon-form-new-post__right-action"
            onClick={handleSubmit}
            disabled={!!caption && caption.length > 500}
          >
            {visibility === 'public_local' || visibility === 'public_federated'
              ? 'Publish!'
              : 'Publish'}
          </Button>
        </div>
      )}
    </>
  )
}
