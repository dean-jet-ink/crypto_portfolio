'use client';

import React from 'react';
import { Provider } from '../../types';
import { Loading } from '@/components/Loading';
import { useLogin } from '../../api/react-query/useLogin';

type Props = {
	provider: Provider;
	disabled: boolean;
};

export const AuthButton = ({ provider, disabled }: Props) => {
	const { loginMutation, loginIsPending } = useLogin();

	return (
		<button
			type="button"
			className={`bg-transparent border p-4 ${!disabled && 'cursor-pointer'}`}
			onClick={() => loginMutation(provider.name)}
			disabled={disabled}
		>
			{loginIsPending ? (
				<div className="m-auto w-8 h-8">
					<Loading />
				</div>
			) : (
				<img src={provider.icon} alt={provider.name} className="w-8 h-8" />
			)}
		</button>
	);
};
