import { Container } from '@/components/Container';
import { AuthButtonList } from '@/features/auth/components/AuthButtonList';
import { LoginForm } from '@/features/auth/components/LoginForm';
import React from 'react';

const Login = () => {
	return (
		<div>
			<Container className="p-4 text-center">
				<div className="mb-8">
					<LoginForm />
				</div>
				<div>
					<div className="flex items-center mb-6">
						<div className="border-b w-full"></div>
						<div className="text-zinc-400 text-sm w-full">Login with</div>
						<div className="border-b w-full"></div>
					</div>
					<AuthButtonList />
				</div>
			</Container>
		</div>
	);
};

export default Login;
