import { NEXT_API_URL } from '@/config/env';
import axios from 'axios';

export const nextAPIAxios = axios.create({
	baseURL: NEXT_API_URL,
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});
