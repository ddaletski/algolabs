#pragma once
#include <algorithm>
#include <cstdint>
#include <iostream>  // TODO: REMOVE
#include <optional>

namespace {
template <typename K, typename V>
struct Node {
    Node* left = nullptr;
    Node* right = nullptr;
    K key;
    V value;
    uint16_t height = 0;

    Node(K key, V value) : key(key), value(value) {
    }

    static Node* insert(Node* node, K key, V value, std::optional<V>& old) {
        if (node == nullptr) {
            return new Node(key, value);
        }

        if (key == node->key) {
            old = std::move(node->value);
            node->value = value;
            return node;
        } else if (key < node->key) {
            node->left = insert(node->left, key, value, old);
        } else {
            node->right = insert(node->right, key, value, old);
        }

        node->recalc_height();

        int balance = get_balance(node);

        if (balance < -1 && key < node->left->key) {
            return rotate_right(node);
        }

        if (balance < -1 && key > node->left->key) {
            node->left = rotate_left(node->left);
            return rotate_right(node);
        }

        if (balance > 1 && key > node->right->key) {
            return rotate_left(node);
        }

        if (balance > 1 && key < node->right->key) {
            node->right = rotate_right(node->right);
            return rotate_left(node);
        }

        return node;
    }

    static int get_height(Node* node) {
        if (node == nullptr) {
            return -1;
        } else {
            return node->height;
        }
    }

    static int get_balance(Node* node) {
        if (node == nullptr) {
            return 0;
        }
        return (get_height(node->right) - get_height(node->left));
    }

    static Node* remove_rightmost_leaf(Node* root, Node* leaf) {
        if (root == leaf) {
            auto left = leaf->left;
            delete leaf;
            return left;
        }

        root->right = remove_rightmost_leaf(root->right, leaf);

        root->recalc_height();
        int balance = get_balance(root);

        if (balance < -1 && get_balance(root->left) <= 0) {
            return rotate_right(root);
        }

        if (balance < -1 && get_balance(root->left) > 0) {
            root->left = rotate_left(root->left);
            return rotate_right(root);
        }

        return root;
    }

    static Node* remove(Node* node, const K& key, std::optional<V>& old) {
        if (node == nullptr) {
            return nullptr;
        }

        if (key < node->key) {
            node->left = remove(node->left, key, old);
        } else if (key > node->key) {
            node->right = remove(node->right, key, old);
        } else {
            old = std::move(node->value);

            if (!node->left) {
                auto right = node->right;
                delete node;
                return right;
            }
            if (!node->right) {
                auto left = node->left;
                delete node;
                return left;
            }

            Node* max_left = node->left->find_max();
            node->key = std::move(max_left->key);
            node->value = std::move(max_left->value);

            node->left = remove_rightmost_leaf(node->left, max_left);
        }

        node->recalc_height();
        int balance = get_balance(node);

        if (balance < -1 && get_balance(node->left) <= 0) {
            return rotate_right(node);
        }

        if (balance < -1 && get_balance(node->left) > 0) {
            node->left = rotate_left(node->left);
            return rotate_right(node);
        }

        if (balance > 1 && get_balance(node->right) >= 0) {
            return rotate_left(node);
        }

        if (balance > 1 && get_balance(node->right) < 0) {
            node->right = rotate_right(node->right);
            return rotate_left(node);
        }

        return node;
    }

    Node* find_max() {
        if (!right) {
            return this;
        }

        return right->find_max();
    }

    static Node* find(Node* node, const K& key) {
        if (node == nullptr) {
            return nullptr;
        }

        if (key == node->key) {
            return node;
        } else if (key < node->key) {
            return find(node->left, key);
        } else {
            return find(node->right, key);
        }
    }

    static Node* rotate_right(Node* node) {
        Node* l = node->left;
        Node* lr = l->right;

        l->right = node;
        node->left = lr;

        node->recalc_height();
        l->recalc_height();

        return l;
    }

    static Node* rotate_left(Node* node) {
        Node* r = node->right;
        Node* rl = r->left;

        r->left = node;
        node->right = rl;

        node->recalc_height();
        r->recalc_height();

        return r;
    }

    void recalc_height() {
        height = 1 + std::max(get_height(left), get_height(right));
    }

#ifdef TESTING
    static bool is_balanced(Node* node) {
        if (node == nullptr) {
            return true;
        }

        if (!is_balanced(node->left) || !is_balanced(node->right)) {
            return false;
        }

        return abs(get_balance(node)) <= 1;
    }

    static bool is_height_consistent(Node* node) {
        if (node == nullptr) {
            return true;
        }

        if (!is_height_consistent(node->left) || !is_height_consistent(node->right)) {
            return false;
        }

        return node->height == 1 + std::max(get_height(node->left), get_height(node->right));
    }
#endif
};
}  // namespace

template <typename K, typename V>
class AVLTree {
  public:
    std::optional<V> insert(K key, V value) {
        auto old = std::optional<V>{};

        root = Node<K, V>::insert(root, key, value, old);

        if (!old) {
            _size++;
        }

        return old;
    }

    std::optional<std::reference_wrapper<V>> find(const K& key) const {
        if (root == nullptr) {
            return std::nullopt;
        }

        auto node = Node<K, V>::find(root, key);
        if (node) {
            return node->value;
        } else {
            return std::nullopt;
        }
    }

    std::optional<V> remove(const K& key) {
        auto old = std::optional<V>{};

        root = Node<K, V>::remove(root, key, old);

        if (old) {
            _size--;
        }

        return old;
    }

    size_t size() const {
        return _size;
    }

#ifdef TESTING
    bool is_balanced() const {
        return Node<K, V>::is_height_consistent(root) && Node<K, V>::is_balanced(root);
    }
#endif

  private:
    Node<K, V>* root;
    size_t _size;
};