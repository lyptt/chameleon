import { ReactNode, useState } from 'react'
import cx from 'classnames'
import Select, { FormatOptionLabelMeta } from 'react-select'
import { IoChevronDown } from 'react-icons/io5'
import { FieldProps } from 'formik'
import { cdnUrl } from '@/core/utils'

export interface FancySelectOption {
  title: string
  value: any
  icon?: string | JSX.Element
}

export interface FancySelectProps extends FieldProps {
  className?: string
  value?: any
  options?: FancySelectOption[]
}

const renderOption: (
  data: FancySelectOption,
  formatOptionLabelMeta: FormatOptionLabelMeta<FancySelectOption>
) => ReactNode = (data) => (
  <>
    {!!data.icon && (
      <>
        {typeof data.icon === 'string' && (
          <img
            className="orbit-fancy-select__option-input-icon"
            src={cdnUrl(data.icon)}
            alt={data.title}
          />
        )}
        {typeof data.icon !== 'string' && data.icon}
      </>
    )}
    {data.title}
  </>
)

export default function FancySelect({
  className,
  value,
  options,
  field,
  form,
  ...rest
}: FancySelectProps) {
  const [enterDisabled, setEnterDisabled] = useState(true)
  return (
    <Select
      className={cx('orbit-fancy-select', className)}
      options={options}
      value={
        options ? options.find((option) => option.value === field.value) : ''
      }
      onChange={(option: FancySelectOption) => {
        form.setFieldValue(field.name, option.value)
      }}
      classNamePrefix="orbit-fancy-select"
      formatOptionLabel={renderOption}
      isSearchable={false}
      onKeyDown={(e) => {
        if (e.key === 'Enter' && enterDisabled) {
          e.preventDefault()
        }
      }}
      {...(rest as any)}
      onMenuOpen={() => setEnterDisabled(false)}
      onMenuClose={() => setEnterDisabled(true)}
      components={{
        DropdownIndicator: () => <IoChevronDown />,
      }}
      placeholder="Select an option"
    />
  )
}
