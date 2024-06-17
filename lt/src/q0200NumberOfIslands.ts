export function numIslands(grid: string[][]): number {
    const [row, col] = [grid.length, grid[0].length];
    let result = 0;
    const visited: Map<string, boolean> = new Map();
    const dfs = (i: number, j: number) => {
        if (
            i < 0 ||
            i >= row ||
            j < 0 ||
            j >= col ||
            visited.get(`${i},${j}`) ||
            grid[i][j] === "0"
        ) {
            return;
        }
        visited.set(`${i},${j}`, true);
        const directions = [
            [1, 0],
            [-1, 0],
            [0, 1],
            [0, -1],
        ];
        directions.forEach(([x, y]) => {
            dfs(i + x, j + y);
        });
    };

    for (let i = 0; i < row; i++) {
        for (let j = 0; j < col; j++) {
            if (grid[i][j] === "1" && !visited.get(`${i},${j}`)) {
                dfs(i, j);
                result += 1;
            }
        }
    }
    return result;
}
