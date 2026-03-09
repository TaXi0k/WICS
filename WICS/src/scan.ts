import * as fs from 'fs/promises';
import * as path from 'path';

export interface scanResults {
    mods:string[];
    others:string[];
}

export async function scanDirectory(path:string) {
    const allEntries:string[] = await fs.readdir(path);

    return allEntries.reduce((acc:scanResults, entry:string) => {
        if (entry.endsWith('.jar')) {
            acc.mods.push(entry);
        } else {
            acc.others.push(entry);
        }
        return acc;
    }, { mods: [], others: [] });
}