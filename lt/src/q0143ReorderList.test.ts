import { reorderList } from "./q0143ReorderList";
import { ListNode } from "./util/ListNode";

function createLinkedList(arr: number[]): ListNode | null {
    if (arr.length === 0) return null;
    const head: ListNode = new ListNode(arr[0]);
    let current = head;
    for (let i = 1; i < arr.length; i++) {
        current.next = new ListNode(arr[i]);
        current = current.next;
    }
    return head;
}

function convertLinkedListToList(head: ListNode | null): number[] {
    const arr: number[] = [];
    let current = head;
    while (current !== null) {
        arr.push(current.val);
        current = current.next;
    }
    return arr;
}

test("reorderList", () => {
    let list = createLinkedList([1, 2, 3, 4]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([1, 4, 2, 3]);

    list = createLinkedList([1, 2, 3, 4, 5]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([1, 5, 2, 4, 3]);

    list = createLinkedList([]);
    reorderList(list);
    expect(convertLinkedListToList(list)).toEqual([]);
});
