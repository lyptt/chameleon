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
} from 'react-icons/io5'

export enum IconButtonIcon {
  Like,
  Message,
  Share,
  Save,
}

export interface IIconButtonProps {
  className?: string
  href?: string
  onClick?: () => void
  icon: IconButtonIcon
  title?: string
  active?: boolean
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
  }
}

export default function IconButton({
  className,
  onClick,
  icon,
  title,
  active,
}: IIconButtonProps) {
  return (
    <button
      className={cx(
        'chameleon-icon-button',
        classNames.button,
        className,
        determineStyleClassName(icon),
        { [classNames.active]: active }
      )}
      title={title}
      onClick={onClick}
    >
      {!active && icon === IconButtonIcon.Like && <IoHeartOutline />}
      {active && icon === IconButtonIcon.Like && <IoHeart />}
      {!active && icon === IconButtonIcon.Message && <IoChatbubbleOutline />}
      {active && icon === IconButtonIcon.Message && <IoChatbubble />}
      {!active && icon === IconButtonIcon.Share && <IoPaperPlaneOutline />}
      {active && icon === IconButtonIcon.Share && <IoPaperPlane />}
      {!active && icon === IconButtonIcon.Save && <IoBookmarkOutline />}
      {active && icon === IconButtonIcon.Save && <IoBookmark />}
    </button>
  )
}
