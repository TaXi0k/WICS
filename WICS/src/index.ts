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
        const { mods, others } = await scanDirectory(absolutePath);

        if (mods.length === 0) {
            console.log(pc.green(`✅ No client-only mods found. Server ${pc.bold("should")} be safe and sound.`))
        } else {
            console.log(pc.redBright("⚠️ Found following CLIENT-ONLY mods:"));
            console.log(pc.yellowBright("------------------------------------"));
            mods.forEach((mod: string) => console.log(`- ${mod}`));
            console.log(pc.yellowBright("------------------------------------"));
            console.log(`Total: ${mods.length} client only mods found.`)
        }
        console.log("");console.log("");
        if (others.length === 0) {
            console.log(pc.green(`✅ No non .jar entries found. Server ${pc.bold("should")} be safe and sound.`))
        } else {
            console.log(pc.redBright("⚠️ Found following non .jar entries:"));
            console.log(pc.yellowBright("------------------------------------"));
            others.forEach((file: string) => console.log(`- ${file}`));
            console.log(pc.yellowBright("------------------------------------"));
            console.log(`Total: ${others.length} non .jar entries found.`)
        }
    } catch (error) {
        console.error(`${pc.redBright("❗ An error occurred during directory scan:")}`, (error as Error).message);
    }
}

main();