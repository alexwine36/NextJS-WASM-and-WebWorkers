import { clsx } from 'clsx';
import { ButtonHTMLAttributes, HTMLAttributes } from 'react';
import { IconBaseProps, IconType } from 'react-icons';
import styles from './index.module.css';

export const Button = (props: ButtonHTMLAttributes<HTMLButtonElement>) => {
	return <button className={styles.button} type="button" {...props} />;
};

type IconButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
	// icon: React.ReactNode;
	Icon: IconType;
	iconProps?: IconBaseProps;
	isActive?: boolean;
};

export const IconButton = ({ isActive, Icon, iconProps, ...props }: IconButtonProps) => {
	return (
		<button
			className={clsx(styles.iconButton, { [styles.active]: isActive })}
			type="button"
			{...props}
		>
			{/* {icon} */}
			<Icon {...iconProps} />
		</button>
	);
};

export const ButtonGroup = ({
	children,
	...props
}: { children: React.ReactNode } & HTMLAttributes<HTMLDivElement>) => {
	return (
		<div className={styles.btnGroup} {...props}>
			{children}
		</div>
	);
};
