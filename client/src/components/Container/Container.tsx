import { ReactNode } from 'react';

type Props = {
	children: ReactNode;
	className?: string;
};

export const Container = ({ children, className }: Props) => {
	return (
		<div className={`bg-teal-900 bg-opacity-40 ${className}`}>{children}</div>
	);
};
