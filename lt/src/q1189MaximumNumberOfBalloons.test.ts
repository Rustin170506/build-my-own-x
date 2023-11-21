import { maxNumberOfBalloons } from "./q1189MaximumNumberOfBalloons";

test("maxNumberOfBalloons", () => {
    expect(maxNumberOfBalloons("nlaebolko")).toBe(1);
    expect(maxNumberOfBalloons("loonbalxballpoon")).toBe(2);
    expect(maxNumberOfBalloons("leetcode")).toBe(0);
    expect(maxNumberOfBalloons("balon")).toBe(0);
});
