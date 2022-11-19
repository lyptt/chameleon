import classNames from './IconButton.module.css'
import cx from 'classnames'
import {
  IoHeartOutline,
  IoHeart,
  IoChatbubbleOutline,
  IoChatbubble,
  IoPaperPlaneOutline,
  IoPaperPlane,
  IoBookmarkOutline,
  IoBookmark,
  IoClose,
  IoStarOutline,
  IoStar,
} from 'react-icons/io5'
import dayjs from 'dayjs'

export enum IconButtonIcon {
  Like,
  Message,
  Share,
  Save,
  Close,
}

export interface IIconButtonProps {
  className?: string
  href?: string
  onClick?: () => void
  icon: IconButtonIcon
  title?: string
  active?: boolean
  small?: boolean
}

function determineStyleClassName(icon: IconButtonIcon): string {
  switch (icon) {
    case IconButtonIcon.Like:
      return classNames.like
    case IconButtonIcon.Message:
      return classNames.message
    case IconButtonIcon.Share:
      return classNames.share
    case IconButtonIcon.Save:
      return classNames.save
    case IconButtonIcon.Close:
      return classNames.close
  }
}

export default function IconButton({
  className,
  onClick,
  icon,
  title,
  active,
  small,
}: IIconButtonProps) {
  const date = dayjs()
  const isLoveDay = date.date() === 14 && date.month() === 1
  return (
    <button
      className={cx(
        'chameleon-icon-button',
        classNames.button,
        className,
        determineStyleClassName(icon),
        {
          [classNames.active]: active,
          [classNames.small]: small,
          [classNames.loveIsInTheAir]: isLoveDay,
        }
      )}
      title={title}
      onClick={onClick}
    >
      {!active && icon === IconButtonIcon.Like && isLoveDay && (
        <IoHeartOutline />
      )}
      {active && icon === IconButtonIcon.Like && isLoveDay && <IoHeart />}
      {!active && icon === IconButtonIcon.Like && !isLoveDay && (
        <IoStarOutline />
      )}
      {active && icon === IconButtonIcon.Like && !isLoveDay && <IoStar />}
      {!active && icon === IconButtonIcon.Message && <IoChatbubbleOutline />}
      {active && icon === IconButtonIcon.Message && <IoChatbubble />}
      {!active && icon === IconButtonIcon.Share && <IoPaperPlaneOutline />}
      {active && icon === IconButtonIcon.Share && <IoPaperPlane />}
      {!active && icon === IconButtonIcon.Save && <IoBookmarkOutline />}
      {active && icon === IconButtonIcon.Save && <IoBookmark />}
      {icon === IconButtonIcon.Close && <IoClose />}
    </button>
  )
}
