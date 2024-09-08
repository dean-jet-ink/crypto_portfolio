import React from 'react';

import img from '../../../public/img/blockchain.jpg';
import Image from 'next/image';

type Props = {
	children: React.ReactNode;
};

const AuthLayout = ({ children }: Props) => {
	return (
		<div className="fixed inset-0 flex items-center justify-center container">
			<div className="w-full">
				<div className="text-center mb-8">
					<h1 className="text-2xl">Crypto Portfolio</h1>
				</div>
				{children}
			</div>
		</div>
	);
};

export default AuthLayout;
