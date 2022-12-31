import CreateLayout, {
  CreateForm,
  CreateFormButtons,
  CreateFormFileUpload,
  CreateFormGroup,
  CreateFormSeparator,
} from '@/components/layouts/CreateLayout'
import { HTMLAttributes, useCallback } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  useCreate,
  createActionSubmitOrbit,
} from '@/components/organisms/CreateContext'
import { Formik } from 'formik'
import Head from 'next/head'

interface INewOrbitForm {
  name: string
  description_md: string
  logo_attachments: File[]
  banner_attachments: File[]
}

export default function NewPostPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useCreate()
  const { submitting } = state

  const onSubmit: (values: INewOrbitForm) => Promise<void> = useCallback(
    async (values) => {
      if (submitting) {
        return
      }

      const attachments =
        values.banner_attachments.length && values.logo_attachments.length
          ? [values.logo_attachments[0], values.banner_attachments[0]]
          : []

      createActionSubmitOrbit(
        values,
        attachments,
        session?.access_token,
        dispatch
      )
    },
    [submitting]
  )

  return (
    <CreateLayout className={cx('orbit-page-new-orbit', className)}>
      <Head>
        <title>Orbit</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Formik
        initialValues={{
          name: '',
          description_md: '',
          logo_attachments: [],
          banner_attachments: [],
        }}
        onSubmit={onSubmit}
      >
        <CreateForm title="Start an Orbit">
          <CreateFormGroup
            title="Name"
            id="name"
            name="name"
            placeholder="Terra"
            disabled={submitting}
          />
          <CreateFormGroup
            title="Description"
            id="description_md"
            name="description_md"
            placeholder="**Hello**, world!"
            as="textarea"
            disabled={submitting}
          />
          <CreateFormSeparator />
          <CreateFormFileUpload
            title="Logo image"
            id="logo_attachments"
            name="logo_attachments"
            accept="image/png,image/jpeg"
            detail="Displayed as the logo for your community.\nPlease ensure you select a square image. Non-square images will be cropped."
            disabled={submitting}
            limit={1}
          />
          <CreateFormFileUpload
            title="Banner image"
            id="banner_attachments"
            name="banner_attachments"
            accept="image/png,image/jpeg"
            detail="Displayed at the top of your community page."
            disabled={submitting}
            limit={1}
          />
          <CreateFormButtons
            submitTitle="Create Orbit"
            cancelTitle="Cancel"
            disabled={submitting}
          />
        </CreateForm>
      </Formik>
    </CreateLayout>
  )
}
