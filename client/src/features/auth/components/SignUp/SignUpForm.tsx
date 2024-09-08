import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import React from 'react';

export const SignUpForm = () => {
	return (
		<div className="flex flex-col items-center gap-4">
			<Input type="email" placeholder="Email" />
			<Input type="password" placeholder="Password" />
			<Button className="w-full">SignUp</Button>
		</div>
	);
};
