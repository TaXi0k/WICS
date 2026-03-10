import * as fs from 'fs/promises';
import pc from 'picocolors';

import { extractModData } from './utils.js';

export interface scanResult {
    clientMods: Record<string, string>;
    nonMods: string[];
}

export async function scanDirectory(path:string): Promise<scanResult> {
    const allEntries:string[] = await fs.readdir(path);
    const clientMods:Record<string, string> = {};
    const mods:string[] = [];
    const others:string[] = [];

    //Filter into mod and non mod files
    allEntries.forEach((entry:string) => {
        if (entry.endsWith('.jar')) {
            mods.push(entry);
        } else {
            others.push(entry);
        }
    });

    //Fetch mod data of all mods found
    mods.forEach((mod:string) => {
        const modData = extractModData(mod, path);

        if (modData) {
            const { id, environment } = modData;
            console.log(`${pc.blueBright("🔹Found mod:")} ${id} (${environment})`);
            if (environment == 'client') {
                clientMods[mod] = id;
            }
        } else {
            console.log(pc.yellow(`⚠️ Skipping: ${mod} (not a valid mod .jar)`));
        }
    });

    return {
        clientMods: clientMods,
        nonMods: others
    };
}