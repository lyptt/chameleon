import CreateLayout, {
  CreateForm,
  CreateFormButtons,
  CreateFormFileUpload,
  CreateFormGroup,
  CreateFormRadioGroup,
  CreateFormSeparator,
} from '@/components/layouts/CreateLayout'
import { HTMLAttributes } from 'react'
import cx from 'classnames'
import { useRouter } from 'next/router'
import { useCreate } from '@/components/organisms/CreateContext'
import { Form, Formik } from 'formik'
import { AccessType, INewPost } from '@/core/api'
import AsidePlaceholder from '@/components/quarks/AsidePlaceholder'
import { IoEarthOutline, IoHomeOutline } from 'react-icons/io5'
import Head from 'next/head'

interface INewOrbitPostForm extends INewPost {
  attachments: File[]
  content_warning?: string
}

export default function NewOrbitPostPage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const router = useRouter()
  const orbitShortcode = router.query.orbitShortcode as string
  const { state } = useCreate()
  const { initialized, submitting, submittingFailed, orbit } = state

  const onSubmit: (values: INewOrbitPostForm) => Promise<void> = async (
    values
  ) => {
    console.log(values)
  }

  return (
    <CreateLayout
      className={cx('orbit-page-new-orbit-post', className)}
      orbitShortcode={orbitShortcode}
    >
      <Head>
        <title>Orbit</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {!orbit && <AsidePlaceholder rows={10} />}
      {!!orbit && (
        <Formik
          initialValues={{
            title: '',
            content_md: '',
            visibility: AccessType.PublicFederated,
            orbit_id: orbit.orbit_id,
            attachments: [],
          }}
          onSubmit={onSubmit}
        >
          <CreateForm title="New Post">
            <CreateFormGroup
              title="Orbit"
              id="orbit_name"
              name="orbit_name"
              placeholder="/o/..."
              hidden
            />
            <CreateFormGroup
              title="Post Title"
              id="title"
              name="title"
              placeholder="Title"
            />
            <CreateFormGroup
              title="Content"
              id="content_md"
              name="content_md"
              placeholder="**Hello**, world!"
              as="textarea"
            />
            <CreateFormFileUpload
              title="Attachments"
              id="attachments"
              name="attachments"
              accept="image/png,image/jpeg"
              multiple
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
            />
            <CreateFormGroup
              title="Sensitive Content"
              id="content_warning"
              name="content_warning"
              detail="If your post contains sensitive material that may negatively impact this community, you can enter a disclaimer
              here. Posts with sensitive content disclaimers are hidden until users choose to view the content in the post."
            />
            <CreateFormButtons submitTitle="Submit Post" cancelTitle="Cancel" />
          </CreateForm>
        </Formik>
      )}
    </CreateLayout>
  )
}
