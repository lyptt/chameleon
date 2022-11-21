import cx from 'classnames'
import {
  IoHeart,
  IoBookmark,
  IoClose,
  IoStar,
  IoEllipsisVertical,
  IoArrowUndo,
  IoRepeat,
  IoEllipsisHorizontal,
  IoShareSocial,
} from 'react-icons/io5'
import dayjs from 'dayjs'

export enum IconButtonIcon {
  Like,
  Reply,
  Boost,
  Save,
  Share,
  Close,
  Options,
  OptionsHorizontal,
}

export interface IIconButtonProps {
  className?: string
  contentClassName?: string
  href?: string
  onClick?: () => void
  icon: IconButtonIcon
  title?: string
  active?: boolean
  small?: boolean
  children?: any
}

function determineStyleClassName(icon: IconButtonIcon): string {
  switch (icon) {
    case IconButtonIcon.Like:
      return 'chameleon-icon-button--like'
    case IconButtonIcon.Reply:
      return 'chameleon-icon-button--reply'
    case IconButtonIcon.Boost:
      return 'chameleon-icon-button--boost'
    case IconButtonIcon.Save:
      return 'chameleon-icon-button--save'
    case IconButtonIcon.Share:
      return 'chameleon-icon-button--share'
    case IconButtonIcon.Close:
      return 'chameleon-icon-button--close'
    case IconButtonIcon.Options:
    case IconButtonIcon.OptionsHorizontal:
      return 'chameleon-icon-button--options'
  }
}

export default function IconButton({
  className,
  contentClassName,
  onClick,
  icon,
  title,
  active,
  small,
  children,
}: IIconButtonProps) {
  const date = dayjs()
  const isLoveDay = date.date() === 14 && date.month() === 1
  return (
    <button
      className={cx(
        'chameleon-icon-button',

        className,
        determineStyleClassName(icon),
        {
          'chameleon-icon-button--active': active,
          'chameleon-icon-button--small': small,
          'chameleon-icon-button--love-is-in-the-air': isLoveDay,
        }
      )}
      title={title}
      onClick={onClick}
    >
      {icon === IconButtonIcon.Like && isLoveDay && (
        <IoHeart
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Like && !isLoveDay && (
        <IoStar
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Reply && (
        <IoArrowUndo
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Boost && (
        <IoRepeat
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Save && (
        <IoBookmark
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Share && (
        <IoShareSocial
          className={cx('chameleon-icon-button__share', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Close && (
        <IoClose
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Options && (
        <IoEllipsisVertical
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.OptionsHorizontal && (
        <IoEllipsisHorizontal
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {children}
    </button>
  )
}
