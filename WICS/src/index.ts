#!/usr/bin/env node

import { scanDirectory } from './scan.js';
import path from 'path';
import pc from 'picocolors';

async function main() {
    const targetPath: string = process.argv[2];

    if (!targetPath) {
        console.log(pc.red("❌ Error: Please provide a directory path."));
        console.log("Usage: wics <path-to-mods-directory>");
        process.exit(1);
    }

    const absolutePath: string = path.resolve(targetPath);
    
    console.log(`${pc.blue(`🔍 WICS scanning:`)} ${absolutePath}\n`);

    try {
        const { clientMods, nonMods } = await scanDirectory(absolutePath);

        console.log('');
        if (Object.keys(clientMods).length == 0) {
            console.log(pc.green(`✅ No client-only mods found. Server ${pc.bold("should")} be safe and sound.`))
        } else {
            console.log(pc.redBright("⚠️ Found following CLIENT-ONLY mods:"));
            console.log(pc.yellowBright("------------------------------------"));
            for (const [file, modID] of Object.entries(clientMods)) { console.log(`▪ ${file} (${modID})`); }
            console.log(pc.yellowBright("------------------------------------"));
            console.log(`Total: ${Object.keys(clientMods).length} client only mods found.`)
        }
        console.log("");console.log("");
        if (nonMods.length === 0) {
            console.log(pc.green(`✅ No non .jar entries found. Server ${pc.bold("should")} be safe and sound.`))
        } else {
            console.log(pc.redBright("⚠️ Found following non .jar entries:"));
            console.log(pc.yellowBright("------------------------------------"));
            nonMods.forEach((file: string) => console.log(`▪ ${file}`));
            console.log(pc.yellowBright("------------------------------------"));
            console.log(`Total: ${nonMods.length} non .jar entries found.`)
        }
    } catch (error) {
        console.error(`${pc.redBright("❗ An error occurred:")}`, (error as Error).message);
    }
}

main();