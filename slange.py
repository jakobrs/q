from typing import Optional, Self, Tuple, Iterator
import random


class Node:
    value: int
    priority: int
    count: int
    sum: int
    left: Optional[Self]
    right: Optional[Self]
    parent: Optional[Self]

    def __init__(self, value: int):
        self.value = value
        self.priority = random.randint(0, 10000000)
        self.count = 1
        self.sum = value
        self.left = None
        self.right = None
        self.parent = None

    def update(self):
        self.count = 1
        self.sum = self.value
        if self.left is not None:
            self.count += self.left.count
            self.sum += self.left.sum
            self.left.parent = self
        if self.right is not None:
            self.count += self.right.count
            self.sum += self.right.sum
            self.right.parent = self


def count(node: Optional[Node]) -> int:
    if node is None:
        return 0
    else:
        return node.count


class Treap:
    root: Optional[Node]

    def __init__(self):
        self.root = None

    def merge(self, left: Optional[Node], right: Optional[Node]) -> Optional[Node]:
        if left is None:
            return right
        if right is None:
            return left

        if left.priority > right.priority:
            left.right = self.merge(left.right, right)
            left.update()
            return left
        else:
            right.left = self.merge(left, right.left)
            right.update()
            return right

    def split(self, against: int) -> Tuple[Optional[Node], Optional[Node]]:
        def go(node: Optional[Node], against: int) -> Tuple[Optional[Node], Optional[Node]]:
            if node is None:
                return None, None

            if node.value < against:
                rl, rr = go(node.right, against)
                node.right = rl
                node.update()
                return node, rr
            else:
                ll, lr = go(node.left, against)
                node.left = lr
                node.update()
                return ll, node

        return go(self.root, against)

    def insert(self, value: int) -> Node:
        new_node = Node(value)
        left, right = self.split(value)
        self.root = self.merge(left, new_node)
        self.root = self.merge(self.root, right)
        return new_node

    def find(self, against: int) -> Optional[Node]:
        def go(node: Optional[Node], against: int) -> Optional[Node]:
            if node is None:
                return None

            if node.value == against:
                return node
            elif node.value > against:
                return go(node.left, against)
            else:
                return go(node.right, against)

        return go(self.root, against)

    def remove(self, value: int):
        node = self.find(value)

        cur = node.parent
        while cur != None:
            cur.count -= 1
            cur.sum -= value
            cur = cur.parent

        merged = self.merge(node.left, node.right)
        if merged is not None:
            merged.parent = node.parent
        if node.parent is not None:
            if node.parent.left == node:
                node.parent.left = merged
            else:
                node.parent.right = merged
        else:
            self.root = merged

    def sum_of_n_greatest(self, n: int) -> int:
        def go(node: Optional[Node], n: int) -> int:
            if node is None or n == 0:
                return 0

            if n >= node.count:
                return node.sum

            total = 0
            right_count = count(node.right)
            total += go(node.right, n)
            n -= right_count
            if n <= 0:
                return total
            total += node.value
            n -= 1
            if n <= 0:
                return total
            total += go(node.left, n)
            return total

        return go(self.root, n)

    def sum(self) -> int:
        if self.root is None:
            return 0
        else:
            return self.root.sum

    def __iter__(self) -> Iterator[Node]:
        def go(node: Optional[Node]) -> Iterator[Node]:
            if node is not None:
                yield from go(node.left)
                yield node
                yield from go(node.right)

        return go(self.root)

    def __len__(self) -> int:
        return count(self.root)


def vis(node: Node, prefix: str):
    if node is not None:
        print(f"{prefix}value: {node.value}")
        print(f"{prefix}count: {node.count}")
        print(f"{prefix}sum: {node.sum}")
        print(f"{prefix}priority: {node.priority}")
        print(f"{prefix}left:")
        vis(node.left, prefix + "  ")
        print(f"{prefix}right:")
        vis(node.right, prefix + "  ")


n, l, k = map(int, input().split())
lst = list(map(int, input().split()))

set = Treap()

for i in range(l):
    set.insert(lst[i])

best = set.sum() - set.sum_of_n_greatest(k)
for i in range(n - l):
    set.remove(lst[i])
    set.insert(lst[i + l])
    here = set.sum() - set.sum_of_n_greatest(k)
    if here < best:
        best = here

print(best)
