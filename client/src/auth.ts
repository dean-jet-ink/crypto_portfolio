import NextAuth, { NextAuthConfig } from 'next-auth';
import Apple from 'next-auth/providers/apple';
import Google from 'next-auth/providers/google';
import { AUTH_GOOGLE_ID, AUTH_GOOGLE_SECRET } from './config/env';
import {
	AUTH_ROUTES,
	DEFAULT_LOGIN_REDIRECT,
	UNAUTHORIZED_REDIRECT,
} from './config/routes';

export const config: NextAuthConfig = {
	providers: [
		Google({ clientId: AUTH_GOOGLE_ID, clientSecret: AUTH_GOOGLE_SECRET }),
		Apple,
	],
	basePath: '/api/auth',
	callbacks: {
		authorized: ({ request: { nextUrl }, auth }) => {
			try {
				const isLoggedIn = !!auth;
				const isAuthRoute = AUTH_ROUTES.includes(nextUrl.pathname);

				if (isAuthRoute) {
					if (isLoggedIn) {
						return Response.redirect(new URL(DEFAULT_LOGIN_REDIRECT, nextUrl));
					}

					return true;
				}

				if (!isAuthRoute && !isLoggedIn) {
					return Response.redirect(new URL(UNAUTHORIZED_REDIRECT, nextUrl));
				}

				return true;
			} catch (err) {
				console.log(err);
			}
		},
		jwt: ({ token, trigger, session }) => {
			return token;
		},
		signIn: async ({ user, account, profile, email, credentials }) => {
			return true;
		},
		redirect: async ({ url, baseUrl }) => {
			return baseUrl;
		},
	},
};

export const { handlers, auth, signIn, signOut } = NextAuth(config);
