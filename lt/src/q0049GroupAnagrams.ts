export function groupAnagrams(strs: string[]): string[][] {
    const strsMap: Map<string, string[]> = new Map();
    for (const str of strs) {
        const key = getKey(str).toString();
        if (strsMap.get(key)) {
            strsMap.get(key)!.push(str);
        } else {
            strsMap.set(key, [str]);
        }
    }

    return Array.from(strsMap.values());
}

function getKey(s: string): number[] {
    const count = new Array(26).fill(0);
    for (const c of s) {
        count[c.charCodeAt(0) - "a".charCodeAt(0)] += 1;
    }

    return count;
}
