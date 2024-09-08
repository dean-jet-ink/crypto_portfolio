'use client';

import React, { useEffect, useState } from 'react';
import { AuthButton } from '../AuthButton';
import { Provider } from '../../types';

const providers: Provider[] = [
	{
		name: 'google',
		icon: '/img/google.svg',
	},
	{
		name: 'apple',
		icon: '/img/apple.svg',
	},
];

export const AuthButtonList = () => {
	const [isProcessing, setIsProcessing] = useState(false);

	const handleClick = () => {
		setIsProcessing(true);
	};

	useEffect(() => {
		return setIsProcessing(false);
	}, []);

	return (
		<div className="flex items-stretch justify-center gap-6">
			{providers.map((provider) => (
				<div key={provider.name} onClick={handleClick}>
					<AuthButton provider={provider} disabled={isProcessing} />
				</div>
			))}
		</div>
	);
};
