class BSTNode:
    def __init__(self, val, l=None, r=None):
        self.val = val
        self.left = l
        self.right = r

class BST:
    def __init__(self, comparator=lambda a, b: a < b):
        self._root = None
        self._comparator = comparator

    def insert(self, val):
        def _insert(at: BSTNode, what: BSTNode):
            if at is None:
                return what
            if self._comparator(what.val, at.val):
                at.left = _insert(at.left, what)
            elif self._comparator(at.val, what.val):
                at.right = _insert(at.right, what)
            return at

        new_node = BSTNode(val)
        self._root = _insert(self._root, new_node)


    def find(self, val):
        def _find(at: BSTNode):
            if at is None:
                raise KeyError()
            if self._comparator(val, at.val):
                return _find(at.left)
            elif self._comparator(at.val, val):
                return _find(at.right)
            else:
                return at.val
            
        return _find(self._root)


    def preorder(self):
        def _f(node):
            if node is None:
                return
            yield node.val
            if node.left is not None:
                yield from _f(node.left)
            if node.right is not None:
                yield from _f(node.right)

        return _f(self._root)
                
            
    def inorder(self):
        def _f(node):
            if node is None:
                return
            if node.left is not None:
                yield from _f(node.left)
            yield node.val
            if node.right is not None:
                yield from _f(node.right)

        return _f(self._root)

            
    def outorder(self):
        def _f(node):
            if node is None:
                return
            if node.right is not None:
                yield from _f(node.right)
            yield node.val
            if node.left is not None:
                yield from _f(node.left)

        return _f(self._root)


    def postorder(self):
        def _f(node):
            if node is None:
                return
            if node.left is not None:
                yield from _f(node.left)
            if node.right is not None:
                yield from _f(node.right)
            yield node.val

        return _f(self._root)