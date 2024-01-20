import { encode, decode } from "./q0271EncodeAndDecodeStrings";

describe("EncodeAndDecodeStrings", () => {
    test("EncodeAndDecodeStrings", () => {
        const list = ["hello", "world"];
        const encoded = encode(list);
        const decoded = decode(encoded);
        expect(decoded).toEqual(list);

        const list2 = ["", "hello", "world"];
        const encoded2 = encode(list2);
        const decoded2 = decode(encoded2);
        expect(decoded2).toEqual(list2);
    });
});
