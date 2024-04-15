class Node:
    def __init__(self, elem):
        self.elem = elem
        self.count = 1
        self.right = None
        self.left = None


class Three:
    def __init__(self):
        self.root = None

    def add(self, elem):
        if self.root is None:
            self.root = Node(elem)
        else:
            self._insert(self.root, elem)

    def _insert(self, nodo: Node, elem):
        if elem > nodo.elem:
            if nodo.right is None:
                nodo.right = Node(elem)
            else:
                self._insert(nodo.right, elem)
        elif elem < nodo.elem:
            if nodo.left is None:
                nodo.left = Node(elem)
            else:
                self._insert(nodo.left, elem)
        else:
            nodo.count += 1

    def print_right(self):
        return self._print(self.root)

    def _print(self, nodo):
        if nodo is None:
            return
        print(f"elem: {nodo.elem}, count: {nodo.count}")
        if nodo.right:
            self._print(nodo.right)

        if nodo.left:
            self._print(nodo.left)


three = Three()

three.add(1)
three.add(2)
three.add(2)
three.add(0)
three.add(-1)

three.print_right()
