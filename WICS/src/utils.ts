import * as path from 'path';
import pc from 'picocolors';
import AdmZip from "adm-zip";

export interface ModData {
    id: string;
    environment: 'client' | 'server' | 'both';
}

const loaders = [
        {
            name: 'fabric/quilt',
            file: 'fabric.mod.json',
            idKey: 'id',
            envKey: 'environment'
        },
        {
            name: 'forge',
            file: 'META-INF/mods.toml',
            idRegex: /modId\s*=\s*["']([^"']+)["']/,
            envRegex: /side\s*=\s*["'](SERVER|CLIENT|BOTH)["']/i
        },
        {
            name: 'neoforge',
            file: 'META-INF/neoforge.mods.toml', // <-- ADD THIS
            idRegex: /modId\s*=\s*["']?([^"'\s]+)["']?/,
            envRegex: /side\s*=\s*["']?([^"'\s]+)["']?/i
        }
];


export function extractModData(mod:string, absolute:string): ModData | null {
    const absolutePath:string = path.join(absolute, mod);
    
    try {
        const zip = new AdmZip(absolutePath);

        for (const loader of loaders) {
            const entry = zip.getEntry(loader.file);
            if (!entry) continue;

            const content = entry.getData().toString("utf8");

            //FOR FABRIC
            if (loader.idKey && loader.file === 'fabric.mod.json') {
                try {
                    const json = JSON.parse(content);
                    return {
                        id: json[loader.idKey],
                        environment: (json[loader.envKey] || 'both').toLowerCase() as any
                    };
                } catch (e) {
                    continue;
                }
            }

            //FOR FORGE
            if (loader.idRegex) {
                const idMatch = content.match(loader.idRegex);
                const envMatch = content.match(loader.envRegex || "");

                return {
                    id: idMatch ? idMatch[1] : "unknown",
                    environment: envMatch ? envMatch[1].toLowerCase() as any : 'both'
                };
            }

        }
    } catch (e) {
        console.error(`${pc.redBright("❗ An error occurred:")} failed to read jar: ${mod}`);
    }
    return null;
}