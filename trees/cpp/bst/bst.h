#ifndef _AVL_H
#define _AVL_H value

#include <functional>
#include <iostream>  // DEL
#include <memory> 
#include <queue>

namespace bst {

template <class T>
class TreeNode {
public:
    T val;
    TreeNode* left;
    TreeNode* right;

    TreeNode(const T& value, TreeNode* l = nullptr, TreeNode* r = nullptr) {
        val = value;
        left = l;
        right = r;
    }
};

class EmptyTree : std::exception {
public:
    virtual const char* what () const noexcept {
        return "tree is empty";
    }
};

enum traverse_type { PREORDER, POSTORDER, INORDER, OUTORDER, BFS };

template <class T, class Compare = std::less<T>>
class BST {
public:
    // unary operator to apply to each nodevalue
    typedef std::function<void(const T&)> Applicator;

private:
    // unary operator to apply to each node with its id
    typedef std::function<void(int id, const TreeNode<T>&)> NodeApplicator;

    size_t _size;
    TreeNode<T>* _root;
    Compare _cmp;

    void _clean_tree(TreeNode<T>* node) {
        if (node) {
            _clean_tree(node->left);
            _clean_tree(node->right);
            delete node;
        }
    }

    bool _insert(TreeNode<T>* node, const T& val) {
        // place to insert value
        TreeNode<T>** place;
        if (_cmp(val, node->val)) {
            place = &node->left;
        } else if (_cmp(node->val, val)) {
            place = &node->right;
        } else {
            node->val = val;
            return false;
        }

        if (*place) {
            return _insert(*place, val);
        } else {
            *place = new TreeNode<T>(val);
            _size++;
            return true;
        }
    }

    TreeNode<T>** _find(TreeNode<T>** nodePtr, const T& val, int* comparisons=0) {
        TreeNode<T>* node = *nodePtr;
        if (!node)
            return nullptr;

        if(comparisons)
            (*comparisons)++;

        if (_cmp(val, node->val)) {
            return _find(&((*nodePtr)->left), val, comparisons);
        } else if (_cmp(node->val, val)) {
            return _find(&((*nodePtr)->right), val, comparisons);
        } else {
            return nodePtr;
        }
    }

    TreeNode<T>** _find_min(TreeNode<T>** nodePtr) {
        if ((*nodePtr)->left) {
            return _find_min(&((*nodePtr)->left));
        } else {
            return nodePtr;
        }
    }

    TreeNode<T>** _find_max(TreeNode<T>** nodePtr) {
        if ((*nodePtr)->right) {
            return _find_max(&((*nodePtr)->right));
        } else {
            return nodePtr;
        }
    }

    void _preorder(TreeNode<T>* node, int id, const NodeApplicator& f) {
        if (node) {
            f(id, *node);
            _preorder(node->left, id*2, f);
            _preorder(node->right, id*2+1, f);
        }
    }

    void _postorder(TreeNode<T>* node, int id, const NodeApplicator& f) {
        if (node) {
            _postorder(node->left, id*2, f);
            _postorder(node->right, id*2+1, f);
            f(id, *node);
        }
    }

    void _inorder(TreeNode<T>* node, int id, const NodeApplicator& f) {
        if (node) {
            _inorder(node->left, id*2, f);
            f(id, *node);
            _inorder(node->right, id*2+1, f);
        }
    }

    void _outorder(TreeNode<T>* node, int id, const NodeApplicator& f) {
        if (node) {
            _outorder(node->right, id*2+1, f);
            f(id, *node);
            _outorder(node->left, id*2, f);
        }
    }

    void _bfs(TreeNode<T>* node, int id, const NodeApplicator& f) {
        std::queue<TreeNode<T>*> queue;
        queue.push(node);

        while (queue.size()) {
            auto node = queue.front();
            queue.pop();
            if (node) {
                f(id++, *node);
                queue.push(node->left);
                queue.push(node->right);
            }
        }
    }

    void _remove(TreeNode<T>** nodePtr) {
        TreeNode<T>* node = *nodePtr;
        if (!node->left && !node->right) {
            *nodePtr = nullptr;
            delete node;
            _size--;
        } else if (!node->right) {
            TreeNode<T>** max_left = _find_max(&(node->left));
            std::swap(node->val, (*max_left)->val);
            _remove(max_left);
        } else {
            TreeNode<T>** min_right = _find_min(&(node->right));
            std::swap(node->val, (*min_right)->val);
            _remove(min_right);
        }
    }

public:
    BST() {
        _size = 0;
        _root = nullptr;
    }

    virtual ~BST() { _clean_tree(_root); }

    int size() { return _size; }

    bool empty() { return _root == nullptr; }

    bool insert(const T& val) {
        if (_root) {
            return _insert(_root, val);
        } else {
            _root = new TreeNode<T>(val);
            _size++;
            return true;
        }
    }

    bool remove(const T& val) {
        if (empty())
            return false;

        TreeNode<T>** nodePtr = _find(&_root, val);
        if (!nodePtr)
            return false;

        _remove(nodePtr);
        return true;
    }

    int find(const T& val) {
        int comparisons = 0;
        TreeNode<T>** result = _find(&_root, val, &comparisons);
        if (result)
            return comparisons;
        else
            return -comparisons;
    }

    T pop_min() {
        // VALUE, NOT BOOL
        if(empty())
            throw EmptyTree();

        TreeNode<T>** min = _find_min(&_root);
        T val = (*min)->val;

        _remove(min);

        return val;
    }

    T pop_max() {
        if(empty())
            throw EmptyTree();

        TreeNode<T>** max = _find_max(&_root);
        T val = (*max)->val;

        _remove(max);

        return val;
    }

    void traverse(traverse_type ttype, Applicator func) {
        NodeApplicator f = [&](int id, const TreeNode<T>& node) {
            func(node.val);
        };
        switch (ttype) {
            case PREORDER:
                _preorder(_root, 1, f);
                break;
            case POSTORDER:
                _postorder(_root, 1, f);
                break;
            case INORDER:
                _inorder(_root, 1, f);
                break;
            case OUTORDER:
                _outorder(_root, 1, f);
                break;
            case BFS:
                _bfs(_root, 1, f);
            default:
                break;
        }
    }

    void gen_dotfile(std::ostream& out) {
        out << "digraph g { \n";
        out << "node [shape=circle];\n";

        auto add_node = [&](int id, const TreeNode<T>& node) {
            out << "node" << id << "[label=" << node.val << "];\n";
            if (node.left) {
                out << "node" << id << " -> node" << id*2 << ";\n";
            } else  {
                out << "nil" << id*2 << "[label=nil color=\"red\"];\n";
                out << "node" << id << " -> nil" << id*2 << ";\n";
            }

            if (node.right) {
                out << "node" << id << " -> node" << id*2+1 << ";\n";
            } else {
                out << "nil" << id*2+1 << "[label=nil color=\"red\"];\n";
                out << "node" << id << " -> nil" << id*2+1 << ";\n";
            }
        };
        _preorder(_root, 1, add_node);

        out << "}";
    }
};

}
#endif /* ifndef _AVL_H */
