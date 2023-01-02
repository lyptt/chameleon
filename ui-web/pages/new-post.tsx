import CreateLayout, {
  CreateForm,
  CreateFormButtons,
  CreateFormFileUpload,
  CreateFormGroup,
  CreateFormRadioGroup,
  CreateFormSeparator,
} from '@/components/layouts/CreateLayout'
import { HTMLAttributes, useCallback, useMemo } from 'react'
import cx from 'classnames'
import { useAuth } from '@/components/organisms/AuthContext'
import {
  useCreate,
  createActionSubmitPost,
} from '@/components/organisms/CreateContext'
import { AccessType } from '@/core/api'
import { Formik } from 'formik'
import Head from 'next/head'
import { IoEarthOutline, IoHomeOutline, IoListOutline } from 'react-icons/io5'
import { useOrbits } from '@/components/organisms/OrbitContext'

interface INewOrbitPostForm {
  content_md: string
  visibility: AccessType
  orbit_name?: string
  attachments: File[]
  content_warning?: string
}

export default function NewPostPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useCreate()
  const {
    state: { orbits },
  } = useOrbits()
  const { submitting } = state

  const postToOptions = useMemo(
    () => [
      { title: 'My feed', value: undefined, icon: <IoListOutline /> },
      ...(orbits || []).map((orbit) => ({
        title: `o/${orbit.shortcode}`,
        icon: orbit.avatar_uri,
        value: orbit.orbit_id,
      })),
    ],

    [orbits]
  )

  const onSubmit: (values: INewOrbitPostForm) => Promise<void> = useCallback(
    async (values) => {
      if (submitting) {
        return
      }

      createActionSubmitPost(
        { ...values, attachment_count: values.attachments.length },
        values.attachments,
        session?.access_token,
        dispatch
      )
    },
    [submitting, session]
  )

  return (
    <CreateLayout className={cx('orbit-page-new-post', className)}>
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
          title: '',
          content_md: '',
          visibility: AccessType.PublicFederated,
          orbit_id: '',
          attachments: [],
        }}
        onSubmit={onSubmit}
      >
        <CreateForm title="New Post">
          <CreateFormGroup
            title="Post to"
            id="orbit_id"
            name="orbit_id"
            disabled={submitting}
            options={postToOptions}
            type="select"
          />
          <CreateFormGroup
            title="Post Title"
            id="title"
            name="title"
            placeholder="Title"
            disabled={submitting}
          />
          <CreateFormGroup
            title="Content"
            id="content_md"
            name="content_md"
            placeholder="**Hello**, world!"
            as="textarea"
            disabled={submitting}
          />
          <CreateFormFileUpload
            title="Attachments"
            id="attachments"
            name="attachments"
            accept="image/png,image/jpeg"
            multiple
            disabled={submitting}
          />
          <CreateFormSeparator />
          <CreateFormRadioGroup
            title="Visibility"
            id="visibility"
            name="visibility"
            options={[
              {
                title: 'Federated',
                icon: <IoEarthOutline />,
                value: AccessType.PublicFederated,
              },
              {
                title: 'Local',
                icon: <IoHomeOutline />,
                value: AccessType.PublicLocal,
              },
            ]}
            disabled={submitting}
          />
          <CreateFormGroup
            title="Sensitive Content"
            id="content_warning"
            name="content_warning"
            detail="If your post contains sensitive material that may negatively impact this community, you can enter a disclaimer
              here. Posts with sensitive content disclaimers are hidden until users choose to view the content in the post."
            disabled={submitting}
          />
          <CreateFormButtons
            submitTitle="Submit Post"
            cancelTitle="Cancel"
            disabled={submitting}
          />
        </CreateForm>
      </Formik>
    </CreateLayout>
  )
}
