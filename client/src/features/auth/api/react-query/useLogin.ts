import { useMutation } from '@tanstack/react-query';
import { login } from '../actions';

export const useLogin = () => {
	const { mutate: loginMutation, isPending: loginIsPending } = useMutation({
		mutationFn: login,
	});

	return {
		loginMutation,
		loginIsPending,
	};
};
