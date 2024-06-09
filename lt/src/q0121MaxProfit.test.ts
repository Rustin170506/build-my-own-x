import { maxProfit } from "./q0121MaxProfit";

describe("S0121", () => {
    test("S0121", () => {
        expect(maxProfit([7, 1, 5, 3, 6, 4])).toBe(5);
        expect(maxProfit([7, 6, 4, 3, 1])).toBe(0);
        expect(maxProfit([1, 2])).toBe(1);
        expect(maxProfit([2, 1])).toBe(0);
    });
});
