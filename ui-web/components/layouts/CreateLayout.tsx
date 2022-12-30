import {
  DetailedHTMLProps,
  FormHTMLAttributes,
  HTMLProps,
  useEffect,
  useRef,
  useState,
} from 'react'
import cx from 'classnames'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import { useRouter } from 'next/router'
import { Field, FieldAttributes, Form, useField } from 'formik'
import SideNav from '@/components/molecules/SideNav'
import {
  createActionInitialize,
  useCreate,
} from '@/components/organisms/CreateContext'
import { useAuth } from '@/components/organisms/AuthContext'
import InfoCard from '@/components/atoms/InfoCard'
import WelcomeCard from '@/components/atoms/WelcomeCard'
import AsidePlaceholder from '@/components/quarks/AsidePlaceholder'
import Button from '@/components/atoms/Button'
import {
  IoAddOutline,
  IoAlertCircleOutline,
  IoCloseOutline,
} from 'react-icons/io5'

dayjs.extend(dayjsUtc)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface CreateLayoutProps extends HTMLProps<HTMLDivElement> {
  orbitShortcode?: string
}

export interface CreateFormProps
  extends DetailedHTMLProps<
    FormHTMLAttributes<HTMLFormElement>,
    HTMLFormElement
  > {
  title: string
  error?: string
}

export interface CreateFormGroupProps extends FieldAttributes<any> {
  hidden?: boolean
  detail?: string
}

export interface CreateFormButtonsProps extends HTMLProps<HTMLDivElement> {
  submitTitle: string
  cancelTitle: string
}

export interface CreateFormRadioGroupProps extends CreateFormGroupProps {
  options: {
    title: string
    icon: JSX.Element
    value: any
  }[]
}

export function CreateForm({
  title,
  error,
  className,
  ref,
  children,
  ...rest
}: CreateFormProps) {
  return (
    <Form className={cx('orbit-create-layout__form', className)} {...rest}>
      <div className="orbit-create-layout__title">{title}</div>
      {!!error && (
        <p className="orbit-create-layout__error">
          <IoAlertCircleOutline /> {error}
        </p>
      )}
      {children}
    </Form>
  )
}

export function CreateFormGroup({
  id,
  className,
  ref,
  title,
  detail,
  hidden,
  ...rest
}: CreateFormGroupProps) {
  if (hidden) {
    return (
      <Field
        className={cx('orbit-create-layout__form-field', className)}
        id={id}
        type="hidden"
        {...rest}
      />
    )
  }

  return (
    <fieldset className="orbit-create-layout__form-group">
      <label
        className={cx('orbit-create-layout__form-field-label', {
          [`${className}-label`]: !!className,
        })}
        htmlFor={id}
      >
        {title}
      </label>
      {!!detail &&
        detail.split('\\n').map((para, i) => (
          <p key={i} className="orbit-create-layout__form-field-detail">
            {para}
          </p>
        ))}
      <Field
        className={cx('orbit-create-layout__form-field', className)}
        id={id}
        {...rest}
      />
    </fieldset>
  )
}

export function CreateFormRadioGroup({
  id,
  className,
  ref,
  title,
  detail,
  hidden,
  options,
  type,
  name,
  disabled,
  ...rest
}: CreateFormRadioGroupProps) {
  if (hidden) {
    return (
      <Field
        className={cx('orbit-create-layout__form-field', className)}
        id={id}
        type="hidden"
        {...rest}
      />
    )
  }

  return (
    <fieldset className="orbit-create-layout__form-group" id={id}>
      <div
        className={cx('orbit-create-layout__form-field-label', {
          [`${className}-label`]: !!className,
        })}
      >
        {title}
      </div>
      {!!detail &&
        detail.split('\\n').map((para, i) => (
          <p key={i} className="orbit-create-layout__form-field-detail">
            {para}
          </p>
        ))}
      <div className="orbit-create-layout__form-field-radio-group">
        {options.map((option) => (
          <Field
            key={option.title}
            value={option.value}
            name={name}
            {...rest}
            disabled={disabled}
          >
            {({ field, form }: any) => (
              <label
                className={cx(
                  'orbit-create-layout__form-field-radio-container',
                  {
                    'orbit-create-layout__form-field-radio-container--selected':
                      form.values[name] === option.value,
                  },
                  {
                    'orbit-create-layout__form-field-radio-container--disabled':
                      disabled,
                  },
                  className
                )}
              >
                <input
                  {...field}
                  value={option.value}
                  type="radio"
                  className="orbit-create-layout__form-field-radio"
                  disabled={disabled}
                />
                {option.icon}
                {option.title}
              </label>
            )}
          </Field>
        ))}
      </div>
    </fieldset>
  )
}

export function CreateFormButtons({
  className,
  submitTitle,
  cancelTitle,
  disabled,
  ...rest
}: CreateFormButtonsProps) {
  const router = useRouter()

  return (
    <div
      className={cx('orbit-create-layout__form-buttons', className)}
      {...rest}
    >
      <Button
        href={(router.query.from as string) || '/'}
        variant="outline"
        disabled={disabled}
      >
        {cancelTitle}
      </Button>
      <Button type="submit" variant="default" disabled={disabled}>
        {submitTitle}
      </Button>
    </div>
  )
}

export function CreateFormSeparator({
  className,
  ...rest
}: HTMLProps<HTMLDivElement>) {
  return (
    <div
      className={cx('orbit-create-layout__form-separator', className)}
      {...rest}
      aria-hidden="true"
    />
  )
}

interface FileUploadThumbnailProps {
  file: File
  index: number
  onRemove: (index: number) => void
  disabled?: boolean
}

function FileUploadThumbnail({
  file,
  index,
  onRemove,
  disabled,
}: FileUploadThumbnailProps) {
  const [imgSrc, setImgSrc] = useState(transparentPixelUri)

  useEffect(() => {
    setImgSrc(transparentPixelUri)

    var fr = new FileReader()
    function onLoad() {
      if (typeof fr.result !== 'string') {
        return
      }

      setImgSrc(fr.result)
    }

    fr.readAsDataURL(file)
    fr.onload = onLoad

    return () => {
      fr.abort()
    }
  }, [file, setImgSrc])

  return (
    <div className="orbit-create-layout__form-file-upload-thumbnail">
      <img src={imgSrc} alt={`Attachment ${index + 1}`} draggable={false} />
      <button
        className="orbit-create-layout__form-file-upload-delete-button"
        onClick={(e) => {
          e.preventDefault()
          onRemove(index)
        }}
        aria-label={`Remove Attachment ${index + 1}`}
        disabled={disabled}
      >
        <IoCloseOutline />
      </button>
    </div>
  )
}

export function CreateFormFileUpload(props: CreateFormGroupProps) {
  const { id, className, ref, title, detail, type, value, disabled, ...rest } =
    props
  const [field, _meta, helpers] = useField(props as any)
  const inputRef = useRef<HTMLInputElement | null>(null)

  const handleRemoveFile = (index: number) => {
    const newValue = [...field.value]
    newValue.splice(index, 1)
    helpers.setValue(newValue)
  }

  return (
    <fieldset className="orbit-create-layout__form-group">
      <label
        className={cx('orbit-create-layout__form-field-label', {
          [`${className}-label`]: !!className,
        })}
        htmlFor={id}
      >
        {title}
      </label>
      {!!detail &&
        detail.split('\\n').map((para, i) => (
          <p key={i} className="orbit-create-layout__form-field-detail">
            {para}
          </p>
        ))}
      <input
        className={cx('orbit-create-layout__form-file-upload', className)}
        id={id}
        type="file"
        ref={inputRef}
        name={field.name}
        onChange={(e) => {
          helpers.setValue([...field.value, ...((e.target.files as any) || [])])
        }}
        onBlur={field.onBlur}
        disabled={disabled}
        {...rest}
      />
      <div className="orbit-create-layout__form-file-upload-thumbnails">
        {field.value.map((file: any, idx: number) => (
          <FileUploadThumbnail
            key={idx}
            file={file}
            index={idx}
            onRemove={handleRemoveFile}
            disabled={disabled}
          />
        ))}
        <button
          className="orbit-create-layout__form-file-upload-choose-button"
          onClick={(e) => {
            e.preventDefault()
            inputRef.current?.click()
          }}
          disabled={disabled}
        >
          <IoAddOutline />
        </button>
      </div>
    </fieldset>
  )
}

export default function CreateLayout({
  title,
  orbitShortcode,
  className,
  children,
  ...rest
}: CreateLayoutProps) {
  const { state, dispatch } = useCreate()
  const { session } = useAuth()
  const {
    initialized,
    orbit,
    orbitLoading,
    orbitLoadingFailed,
    submittedPost,
  } = state
  const router = useRouter()

  useEffect(() => {
    if (!initialized) {
      createActionInitialize(orbitShortcode, session?.access_token, dispatch)
    }
  }, [initialized, dispatch, orbitShortcode, session])

  useEffect(() => {
    if (!!submittedPost) {
      router.replace(`/feed/${submittedPost.post_id}`)
    }
  }, [submittedPost, router])

  useEffect(() => {
    if (!session) {
      router.replace(`/api/oauth/login`)
    }
  }, [session, router])

  return (
    <section className={cx('orbit-create-layout', className)} {...rest}>
      <SideNav />
      <div className="orbit-create-layout__content">{children}</div>
      {!!orbitShortcode && (orbitLoading || orbitLoadingFailed || !orbit) && (
        <aside className="orbit-create-layout__sidebar">
          <AsidePlaceholder />
        </aside>
      )}
      {!!orbitShortcode && !orbitLoading && !orbitLoadingFailed && !!orbit && (
        <aside className="orbit-create-layout__sidebar">
          <InfoCard
            title="About this community"
            innerHTML={orbit.description_html}
          >
            Created {dayjs.utc(orbit.created_at).format('MMM DD, YYYY')}
          </InfoCard>
        </aside>
      )}
      {!orbitShortcode && (
        <aside className="orbit-page-home__sidebar">
          <WelcomeCard hideActions />
        </aside>
      )}
    </section>
  )
}
