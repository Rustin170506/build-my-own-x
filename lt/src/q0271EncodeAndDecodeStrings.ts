export function encode(list: string[]): string {
    let result = "";
    for (const s of list) {
        result += `${s.length}#` + s;
    }

    return result;
}
export function decode(s: string): string[] {
    const result = [];
    for (let i = 0; i < s.length; ) {
        let length = "";
        let j = i;
        while (s[j] !== "#" && j < s.length) {
            length += s[j];
            j++;
        }
        const l = Number(length);
        result.push(s.slice(j + 1, j + 1 + l));
        i = j + 1 + l;
    }

    return result;
}
