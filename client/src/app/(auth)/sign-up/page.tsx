import { Container } from '@/components/Container';
import { SignUpForm } from '@/features/auth/components/SignUp';
import React from 'react';

const page = () => {
	return (
		<div>
			<Container className="p-4 text-center">
				<div className="mb-8">
					<SignUpForm />
				</div>
			</Container>
		</div>
	);
};

export default page;
