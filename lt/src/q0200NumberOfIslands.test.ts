import { numIslands } from "./q0200NumberOfIslands";

test("q0200NumberOfIslands", () => {
    const grid = [
        ["1", "1", "1", "1", "0"],
        ["1", "1", "0", "1", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "0", "0", "0"],
    ];
    expect(numIslands(grid)).toBe(1);
    const grid2 = [
        ["1", "1", "0", "0", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "1", "0", "0"],
        ["0", "0", "0", "1", "1"],
    ];
    expect(numIslands(grid2)).toBe(3);
});
