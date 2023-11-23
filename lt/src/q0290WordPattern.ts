export function wordPattern(pattern: string, s: string): boolean {
    const words = s.split(" ");
    if (pattern.length !== words.length) {
        return false;
    }
    const patternMap: Map<string, number> = new Map();
    const wordsMap: Map<string, number> = new Map();

    for (let index = 0; index < pattern.length; index++) {
        const word = words[index];
        const c = pattern[index];
        if (!patternMap.has(c)) {
            patternMap.set(c, index);
        }
        if (!wordsMap.has(word)) {
            wordsMap.set(word, index);
        }

        if (patternMap.get(c) !== wordsMap.get(word)) {
            return false;
        }
    }

    return true;
}
