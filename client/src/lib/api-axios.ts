import { API_URL } from '@/config/env';
import axios from 'axios';

export const apiAxios = axios.create({
	baseURL: API_URL,
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json',
	},
});
