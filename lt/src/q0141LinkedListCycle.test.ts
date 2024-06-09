import { hasCycle } from "./q0141LinkedListCycle";
import { ListNode } from "./util/ListNode";

function createLinkedList(arr: number[], pos: number): ListNode | null {
    const dummy = new ListNode(0);
    let tail = dummy;
    let cycleNode = null;

    for (let i = 0; i < arr.length; i++) {
        tail.next = new ListNode(arr[i]);
        tail = tail.next;
        if (i === pos) {
            cycleNode = tail;
        }
    }

    // Create cycle
    if (cycleNode !== null) {
        tail.next = cycleNode;
    }

    return dummy.next;
}

test("hasCycle", () => {
    let list = createLinkedList([3, 2, 0, -4], 1);
    expect(hasCycle(list)).toBe(true);

    list = createLinkedList([1, 2], 0);
    expect(hasCycle(list)).toBe(true);

    list = createLinkedList([1], -1);
    expect(hasCycle(list)).toBe(false);
});
