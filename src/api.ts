import { invoke } from '@tauri-apps/api/core';
import type { Note, NoteColor } from './types';
export const isTauri='__TAURI_INTERNALS__' in window;
const demoNotes:Note[]=[{id:'office',title:'Office',body:'- understand all the APIs\n- create tickets for PRD creation',color:'blue',archived:false,createdAt:'2026-08-29T18:00:00Z',updatedAt:new Date().toISOString(),x:1080,y:250,width:330,height:340,alwaysOnTop:true},{id:'groceries',title:'Groceries',body:'- apple\n- 4x banana\n- dry fruits\n- peanuts',color:'mint',archived:false,createdAt:'2026-08-29T17:00:00Z',updatedAt:new Date().toISOString(),x:1080,y:260,width:330,height:340,alwaysOnTop:true},{id:'side-projects',title:'Side-projects',body:'- understand the architecture of the backend api...',color:'yellow',archived:false,createdAt:'2026-08-29T16:00:00Z',updatedAt:new Date().toISOString()},{id:'hold-my-lid',title:'hold my lid',body:'- work on the clamshell',color:'lilac',archived:false,createdAt:'2026-08-29T08:00:00Z',updatedAt:new Date().toISOString()},{id:'supercmd',title:'supercmd',body:'- work on the custom extension',color:'blue',archived:true,createdAt:'2026-08-29T07:00:00Z',updatedAt:new Date().toISOString()}];
export async function loadNotes(){if(!isTauri){const r=localStorage.getItem('hold-my-notes');if(!r){localStorage.setItem('hold-my-notes',JSON.stringify(demoNotes));return demoNotes}return JSON.parse(r)}return invoke<Note[]>('load_notes')}
export async function saveNotes(notes:Note[]){if(!isTauri){localStorage.setItem('hold-my-notes',JSON.stringify(notes));return}await invoke('save_notes',{notes})}
export async function getNote(id:string){return (await loadNotes()).find(n=>n.id===id)??null}
export async function upsertNote(note:Note){const a=await loadNotes(),i=a.findIndex(n=>n.id===note.id);if(i<0)a.unshift(note);else a[i]=note;await saveNotes(a)}
export async function deleteNote(id:string){await saveNotes((await loadNotes()).filter(n=>n.id!==id))}
export function makeNote(title='Untitled note',body=''):Note{const now=new Date().toISOString();return{id:crypto.randomUUID(),title,body,color:'yellow',archived:false,createdAt:now,updatedAt:now,alwaysOnTop:true,desktopAttached:false}}
export function touch(n:Note,p:Partial<Note>):Note{return{...n,...p,updatedAt:new Date().toISOString()}}
export function colorToClass(c:NoteColor){return`note-${c}`}
