import { groupAnagrams } from "./q0049GroupAnagrams";

test("groupAnagrams", () => {
    expect(groupAnagrams(["a", "a"])).toStrictEqual([["a", "a"]]);
    expect(
        groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]),
    ).toStrictEqual([["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]);
});
