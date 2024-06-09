export function maxNumberOfBalloons(text: string): number {
    const balloonCharacterCounts: Map<string, number> = new Map(
        [
            ["b", 1],
            ["a", 1],
            ["l", 2],
            ["o", 2],
            ["n", 1],
        ].map(([character, count]) => [character as string, count as number]),
    );

    const textCharacterCounts: Map<string, number> = text
        .split("")
        .reduce((accumulator: Map<string, number>, character: string) => {
            accumulator.set(character, (accumulator.get(character) || 0) + 1);
            return accumulator;
        }, new Map());

    let maxBalloonCount = text.length;

    for (const [character, count] of balloonCharacterCounts) {
        if (textCharacterCounts.has(character)) {
            maxBalloonCount = Math.min(
                maxBalloonCount,
                Math.floor(textCharacterCounts.get(character)! / count),
            );
        } else {
            return 0;
        }
    }

    return maxBalloonCount;
}
