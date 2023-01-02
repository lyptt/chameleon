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
import {
  IoAddOutline,
  IoAlertCircleOutline,
  IoCloseOutline,
} from 'react-icons/io5'
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
import Progress from '@/components/quarks/Progress'
import FancySelect from '../atoms/FancySelect'

dayjs.extend(dayjsUtc)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface CreateLayoutProps extends HTMLProps<HTMLDivElement> {
  orbitShortcode?: string
  embedded?: boolean
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
  options?: {
    title: string
    value: any
    icon?: string | JSX.Element
  }[]
}

export interface CreateFormFileUploadGroupProps extends CreateFormGroupProps {
  limit?: number
}

export interface CreateFormButtonsProps extends HTMLProps<HTMLDivElement> {
  submitTitle: string
  cancelTitle: string
  onCancel?: () => void
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
  options,
  type,
  ...rest
}: CreateFormGroupProps) {
  if (hidden) {
    return (
      <Field
        className={cx('orbit-create-layout__form-field-custom', className)}
        id={id}
        type="hidden"
        {...rest}
      />
    )
  }

  if (type === 'select') {
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
        <Field
          id={id}
          className={cx('orbit-create-layout__form-field-custom', className)}
          ref={ref}
          title={title}
          detail={detail}
          hidden={hidden}
          options={options}
          type={type}
          component={FancySelect}
          {...rest}
        />
      </fieldset>
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
        type={type}
        onKeyDown={(e: any) => {
          if (e.key === 'Enter') {
            e.preventDefault()
            e.stopPropagation()
            return false
          }
        }}
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
        {(options || []).map((option) => (
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
                  onKeyDown={(e: any) => {
                    if (e.key === 'Enter') {
                      e.preventDefault()
                      e.stopPropagation()
                      return false
                    }
                  }}
                />
                {typeof option.icon !== 'string' && option.icon}
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
  onCancel,
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
        onClick={
          onCancel
            ? (e) => {
                e.preventDefault()
                onCancel()
              }
            : undefined
        }
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

export function CreateFormFileUpload(props: CreateFormFileUploadGroupProps) {
  const {
    id,
    className,
    ref,
    title,
    detail,
    type,
    value,
    disabled,
    limit,
    ...rest
  } = props
  const [field, _meta, helpers] = useField(props as any)
  const inputRef = useRef<HTMLInputElement | null>(null)
  const [remaining, setRemaining] = useState(limit ? limit : 30)

  const handleRemoveFile = (index: number) => {
    const newValue = [...field.value]
    newValue.splice(index, 1)
    helpers.setValue(newValue)
    setRemaining(remaining + 1)
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
          const newFiles = (e.target.files as any) || []
          helpers.setValue([...field.value, ...newFiles])
          setRemaining(remaining - newFiles.length)
        }}
        onBlur={field.onBlur}
        disabled={disabled || remaining <= 0}
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
        {remaining > 0 && (
          <button
            type="button"
            className="orbit-create-layout__form-file-upload-choose-button"
            onClick={(e) => {
              e.preventDefault()
              inputRef.current?.click()
            }}
            disabled={disabled || remaining <= 0}
          >
            <IoAddOutline />
          </button>
        )}
      </div>
    </fieldset>
  )
}

export default function CreateLayout({
  title,
  orbitShortcode,
  embedded,
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
    submitting,
    submittedPost,
    submittedOrbit,
    submittedProfile,
    submittingImageProgress,
  } = state
  const router = useRouter()

  useEffect(() => {
    if (!initialized && !embedded) {
      createActionInitialize(orbitShortcode, session?.access_token, dispatch)
    }
  }, [initialized, dispatch, orbitShortcode, session, embedded])

  useEffect(() => {
    if (!!submittedPost && !embedded) {
      router.replace(`/feed/${submittedPost.post_id}`)
    }
  }, [submittedPost, router, embedded])

  useEffect(() => {
    if (!!submittedOrbit && !embedded) {
      router.replace(`/orbits/${submittedOrbit.shortcode}`)
    }
  }, [submittedOrbit, router, embedded])

  useEffect(() => {
    if (!!submittedOrbit && !embedded) {
      router.replace(`/orbits/${submittedOrbit.shortcode}`)
    }
  }, [submittedOrbit, router, embedded])

  useEffect(() => {
    if (!!submittedProfile && !embedded) {
      router.replace(`/profile`)
    }
  }, [submittedProfile, router, embedded])

  useEffect(() => {
    if (!session && !embedded) {
      router.replace(`/api/oauth/login`)
    }
  }, [session, router, embedded])

  if (embedded) {
    return (
      <section className={cx('orbit-create-layout', className)} {...rest}>
        <div className="orbit-create-layout__content">{children}</div>
      </section>
    )
  }

  return (
    <section className={cx('orbit-create-layout', className)} {...rest}>
      {submitting && (
        <Progress
          className="orbit-create-layout__progress"
          value={submittingImageProgress}
          max={1}
        />
      )}
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
