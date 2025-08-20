export type Extension = {
	id: string;
	name: string;
	description: string;
	current_version: string;
};

export type Pagination = {
	query: string;
	page: number;
	per_page: number;
};
