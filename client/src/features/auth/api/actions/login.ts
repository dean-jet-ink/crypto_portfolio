'use server';

import { signIn } from '@/auth';

export const login = async (providerName: string) => {
	await signIn(providerName);
};
