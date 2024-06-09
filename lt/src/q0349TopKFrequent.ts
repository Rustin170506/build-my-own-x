export function topKFrequent(nums: number[], k: number): number[] {
    const freq = new Map<number, number>();
    for (const num of nums) {
        freq.set(num, (freq.get(num) ?? 0) + 1);
    }
    const freqArr = Array.from(freq.entries());
    freqArr.sort((a, b) => b[1] - a[1]);
    return freqArr.slice(0, k).map((kv) => kv[0]);
}

export function topKFrequent2(nums: number[], k: number): number[] {
    const freq = new Map<number, number>();
    for (const num of nums) {
        freq.set(num, (freq.get(num) ?? 0) + 1);
    }

    const freqArr: number[][] = Array.from(
        { length: nums.length + 1 },
        () => [],
    );
    for (const [num, count] of freq.entries()) {
        freqArr[count].push(num);
    }

    const result = [];
    for (let i = freqArr.length - 1; i >= 0 && result.length < k; i--) {
        result.push(...freqArr[i]);
    }
    return result.slice(0, k);
}
