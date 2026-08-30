export type NoteColor = 'yellow'|'coral'|'mint'|'blue'|'lilac';
export type Note = {id:string;title:string;body:string;color:NoteColor;archived:boolean;createdAt:string;updatedAt:string;x?:number;y?:number;width?:number;height?:number;monitorId?:string;alwaysOnTop?:boolean;desktopAttached?:boolean};
