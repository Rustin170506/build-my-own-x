export function maxProfit(prices: number[]): number {
    if (prices.length == 1) {
        return 0;
    }
    let maxProfit = 0;
    let [l, r] = [0, 1];

    while (r < prices.length) {
        if (prices[r] - prices[l] > 0) {
            maxProfit = Math.max(maxProfit, prices[r] - prices[l]);
            r += 1;
        } else {
            l = r;
            r += 1;
        }
    }

    return maxProfit;
}
