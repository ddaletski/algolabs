#include "bst/bst.h"
#include <iostream>
#include <fstream>


int main() {
    bst::BST<int> tree;

    std::ifstream infile("input.txt");

    if(!infile.is_open()) {
        return 1;
    }

    int val;
    do {
        infile >> val;
        if (infile.eof()) {
            break;
        }
        tree.insert(val);
    } while(!infile.eof());

    std::ofstream outfile("output.txt");

    tree.traverse(bst::traverse_type::PREORDER,
                  [&](const int &val) {
                      outfile << val << std::endl;
                  });

    return 0;
}