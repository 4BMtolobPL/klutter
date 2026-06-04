export enum ToolStatus {
	READY = 'READY',
	COMING_SOON = 'COMING_SOON',
	DRAFT = 'DRAFT'
}

export interface Tool {
	id: string;
	title: string;
	description: string;
	icon: string;
	color: string;
	status: ToolStatus;
}
