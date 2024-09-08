import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import React from 'react';

export const LoginForm = () => {
	return (
		<form className="flex flex-col items-center gap-4">
			<Input type="email" placeholder="Email" />
			<Input type="password" placeholder="Password" />
			<Button className="w-full">Login</Button>
		</form>
	);
};
