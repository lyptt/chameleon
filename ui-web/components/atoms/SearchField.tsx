import { InputHTMLAttributes } from 'react'
import cx from 'classnames'
import { IoSearch } from 'react-icons/io5'

export default function SearchField({
  id,
  className,
  title,
  placeholder,
  ...props
}: InputHTMLAttributes<HTMLInputElement>) {
  const inputId = id ? `search-field-${id}` : 'search-field'

  return (
    <fieldset id={id} className={cx('chameleon-search-field', className)}>
      <label htmlFor={inputId} className="chameleon-search-field__label">
        {placeholder || title}
      </label>
      <input
        id={inputId}
        className="chameleon-search-field__input"
        placeholder={placeholder || title}
        {...props}
      />
      <IoSearch className="chameleon-search-field__icon" />
    </fieldset>
  )
}
