#include <gtest/gtest.h>
#include <cstdlib>
#include "bst.h"
#include <random>
#include <algorithm>

using namespace bst;

template <class T>
std::ostream& operator << (std::ostream& str, const std::vector<T>& vec) {
    str << "[ ";
    for(auto x : vec) {
        str << x << " ";
    }
    str << "]";
    return str;
}


class TreeTest : public ::testing::Test {
protected:
    virtual void SetUp() {
        random_vector = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
        shuffle (random_vector.begin(), random_vector.end(),
                 std::default_random_engine(0));
    }

    virtual void TearDown() {}

    BST<int> tree1;
    std::vector<int> random_vector;
};


TEST_F(TreeTest, TestInsert) {
    for(auto x : random_vector) {
        ASSERT_TRUE(tree1.insert(x));
    }
    ASSERT_EQ(tree1.size(), random_vector.size());

    for(auto x : random_vector) {
        ASSERT_FALSE(tree1.insert(x));
    }
}

TEST_F(TreeTest, TestRemove) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    for(int i = 1000; i < 1100; ++i) {
        ASSERT_FALSE(tree1.remove(i));
    }

    for(auto x : random_vector) {
        ASSERT_TRUE(tree1.remove(x));
    }

    ASSERT_EQ(tree1.size(), 0);
}

TEST_F(TreeTest, TestFind) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    for(int i = 1; i <= 10; ++i) {
        ASSERT_TRUE(tree1.find(i));
    }

    for(int i = -10; i <= 0; ++i) {
        ASSERT_FALSE(tree1.find(i));
    }

    for(int i = 11; i <= 20; ++i) {
        ASSERT_FALSE(tree1.find(i));
    }

}


TEST_F(TreeTest, TestPopMin) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    int initial_size = tree1.size();

    int min = tree1.pop_min();
    int true_min = *std::min_element(random_vector.begin(),
                                    random_vector.end());
    ASSERT_EQ(min, true_min);
    ASSERT_EQ(tree1.size(), initial_size - 1);
    ASSERT_FALSE(tree1.remove(min));
}

TEST_F(TreeTest, TestPopMax) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    int initial_size = tree1.size();

    int true_max = *std::max_element(random_vector.begin(),
                                    random_vector.end());
    int max = tree1.pop_max();
    ASSERT_EQ(max, true_max);
    ASSERT_EQ(tree1.size(), initial_size - 1);
    ASSERT_FALSE(tree1.remove(max));
}


TEST_F(TreeTest, TestPopMinMaxThrowsIfEmpty) {
    ASSERT_THROW(tree1.pop_max(), EmptyTree);
    ASSERT_THROW(tree1.pop_min(), EmptyTree);
}


TEST_F(TreeTest, TestTraverseMinmax) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    std::vector<int> sorted_vector;

    std::vector<int> true_sorted_vector = random_vector;
    std::sort(true_sorted_vector.begin(), true_sorted_vector.end());

    BST<int>::Applicator func = [&](const int& nodeVal) {
        sorted_vector.push_back(nodeVal);
    };

    tree1.traverse(traverse_type::INORDER, func);

    ASSERT_EQ(sorted_vector, true_sorted_vector);
}


TEST_F(TreeTest, TestTraverseMaxmin) {
    for(auto x : random_vector) {
        tree1.insert(x);
    }

    std::vector<int> sorted_vector;

    std::vector<int> true_sorted_vector = random_vector;
    std::sort(true_sorted_vector.begin(), true_sorted_vector.end());
    std::reverse(true_sorted_vector.begin(), true_sorted_vector.end());

    BST<int>::Applicator func = [&](const int& nodeVal) {
        sorted_vector.push_back(nodeVal);
    };

    tree1.traverse(traverse_type::OUTORDER, func);

    ASSERT_EQ(sorted_vector, true_sorted_vector);
}


TEST_F(TreeTest, TestTraverseDirect) {
    std::vector<int> values = {4, 2, 6, 1, 3, 5, 7};

    for(auto v: values) {
        tree1.insert(v);
    }

    std::vector<int> traverse_result;

    BST<int>::Applicator func = [&](const int& nodeVal) {
        traverse_result.push_back(nodeVal);
    };

    tree1.traverse(traverse_type::PREORDER, func);

    std::vector<int> expected_result = {4, 2, 1, 3, 6, 5, 7};

    ASSERT_EQ(traverse_result, expected_result);
}

TEST_F(TreeTest, TestTraverseReversed) {
    std::vector<int> values = {4, 2, 6, 1, 3, 5, 7};

    for(auto v: values) {
        tree1.insert(v);
    }

    std::vector<int> traverse_result;

    BST<int>::Applicator func = [&](const int& nodeVal) {
        traverse_result.push_back(nodeVal);
    };

    tree1.traverse(traverse_type::POSTORDER, func);

    std::vector<int> expected_result = {1, 3, 2, 5, 7, 6, 4};

    ASSERT_EQ(traverse_result, expected_result);
}

TEST_F(TreeTest, TestTraverseLevels) {
    std::vector<int> values = {4, 2, 6, 1, 3, 5, 7};

    for(auto v: values) {
        tree1.insert(v);
    }

    std::vector<int> traverse_result;

    BST<int>::Applicator func = [&](const int& nodeVal) {
        traverse_result.push_back(nodeVal);
    };

    tree1.traverse(traverse_type::BFS, func);

    std::vector<int> expected_result = values;

    ASSERT_EQ(traverse_result, expected_result);
}


int main(int argc, char* argv[]) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
