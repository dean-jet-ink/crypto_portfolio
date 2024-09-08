export type Provider = {
	name: 'google' | 'apple' | 'twitter';
	icon: string;
};

export type Session = {
	userName: string;
	email: string;
};
