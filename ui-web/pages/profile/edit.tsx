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
  createActionSubmitProfile,
} from '@/components/organisms/CreateContext'
import { Formik } from 'formik'
import Head from 'next/head'
import { useProfile } from '@/components/organisms/ProfileContext'
import AsidePlaceholder from '@/components/quarks/AsidePlaceholder'
import { INewProfile, IProfile } from '@/core/api'

function buildLink(
  title: string | undefined,
  url: string | undefined
): { title: string; url: string } | undefined {
  return title && url ? { title, url } : undefined
}

function buildLinks(profile: IProfile): {
  title: string
  url: string
}[] {
  return [
    buildLink(profile.url_1_title, profile.url_1),
    buildLink(profile.url_2_title, profile.url_2),
    buildLink(profile.url_3_title, profile.url_3),
    buildLink(profile.url_4_title, profile.url_4),
    buildLink(profile.url_5_title, profile.url_5),
  ].filter((v) => !!v) as any
}

export default function EditProfilePage({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()
  const { state, dispatch } = useCreate()
  const {
    state: { profile },
  } = useProfile()
  const { submitting } = state

  const onSubmit: (values: INewProfile) => Promise<void> = useCallback(
    async (values) => {
      if (submitting || !profile) {
        return
      }

      createActionSubmitProfile(
        profile,
        values,
        values.attachments,
        session?.access_token,
        dispatch
      )
    },
    [submitting, profile, session]
  )

  return (
    <CreateLayout className={cx('orbit-page-edit-profile', className)}>
      <Head>
        <title>Orbit - Update Profile</title>
        <meta
          name="description"
          content="Welcome to Orbit, your place to share cool things with the world in an open, federated network"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      {profile && (
        <Formik
          initialValues={{
            handle: profile.handle || '',
            intro_md: profile.intro_md || '',
            email: profile.email || '',
            links: buildLinks(profile),
            attachments: [],
          }}
          onSubmit={onSubmit}
        >
          <CreateForm title="Update Profile">
            <CreateFormGroup
              title="Handle"
              id="handle"
              name="handle"
              detail="This is the handle people use to address you, it doesn't have to match your username and you can change it at any time."
              disabled={submitting}
            />
            <CreateFormGroup
              title="E-mail address"
              id="email"
              name="email"
              disabled={submitting}
            />
            <CreateFormGroup
              title="Password"
              id="password"
              name="password"
              type="password"
              disabled={submitting}
            />
            <CreateFormGroup
              title="About you"
              id="intro_md"
              name="intro_md"
              placeholder="**Hello**, world!"
              as="textarea"
              detail="You can use this space to introduce and write information about yourself."
              disabled={submitting}
            />
            <CreateFormSeparator />
            <CreateFormFileUpload
              title="Avatar image"
              id="attachments"
              name="attachments"
              accept="image/png,image/jpeg"
              disabled={submitting}
              limit={1}
            />
            <CreateFormButtons
              submitTitle="Save Changes"
              cancelTitle="Cancel"
              disabled={submitting}
            />
          </CreateForm>
        </Formik>
      )}
      {!profile && <AsidePlaceholder rows={10} />}
    </CreateLayout>
  )
}
