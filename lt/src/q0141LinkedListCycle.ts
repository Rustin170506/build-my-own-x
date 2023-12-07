import { ListNode } from "./util/ListNode";

function hasCycle(head: ListNode | null): boolean {
    let [slow, fast] = [head, head];

    while (fast?.next?.next) {
        slow = slow!.next;
        fast = fast.next.next;
        if (slow == fast) {
            return true;
        }
    }

    return false;
}
