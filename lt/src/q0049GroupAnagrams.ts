export function groupAnagrams(strs: string[]): string[][] {
    return Array.from(
        strs
            .reduce((acc, str) => {
                const count = new Array(26).fill(0);
                for (const char of str) {
                    count[char.charCodeAt(0) - "a".charCodeAt(0)]++;
                }
                const key = count.toString();
                const group = acc.get(key) || [];
                group.push(str);
                acc.set(key, group);
                return acc;
            }, new Map<string, string[]>())
            .values(),
    );
}
