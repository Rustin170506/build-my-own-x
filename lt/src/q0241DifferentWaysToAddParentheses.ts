export function diffWaysToCompute(expression: string): number[] {
    const dfs = (expression: string, memo: Map<string, number[]>): number[] => {
        if (memo.has(expression)) {
            return memo.get(expression)!;
        }
        const res: number[] = [];
        for (let i = 0; i < expression.length; i++) {
            const char = expression[i];
            if (char === "+" || char === "-" || char === "*") {
                const left = dfs(expression.substring(0, i), memo);
                const right = dfs(expression.substring(i + 1), memo);
                for (const l of left) {
                    for (const r of right) {
                        if (char === "+") {
                            res.push(l + r);
                        } else if (char === "-") {
                            res.push(l - r);
                        } else {
                            res.push(l * r);
                        }
                    }
                }
            }
        }
        if (res.length === 0) {
            res.push(parseInt(expression));
        }
        memo.set(expression, res);
        return res;
    };

    return dfs(expression, new Map<string, number[]>());
}
