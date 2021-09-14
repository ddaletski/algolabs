#include "bst.h"
#include <iostream>
#include <vector>
#include <fstream>

int main(int argc, char* argv[]) {
    if(argc < 3) {
        std::cout << "usage: " << argv[0] << " infile outfile" << std::endl;
        return 0;
    }

    std::ifstream infile(argv[1]);
    std::ofstream outfile(argv[2]);

    if(!infile.is_open()) {
        std::cout << argv[1] << ": file not found" << std::endl;
        return -1;
    } else if(!outfile.is_open()) {
        std::cout << argv[2] << ": can't open file for writing" << std::endl;
        return -1;
    }

    bst::BST<int> tree;

    while(!infile.eof()) {
        char command;
        int arg;

        infile >> command;
        if(infile.eof())
            break;

        command = tolower(command);

        switch (command) {
            case 'i': {
                infile >> arg;
                bool result = tree.insert(arg);
                outfile << arg << ": ";
                if (result)
                    outfile << "inserted";
                else
                    outfile << "not inserted";
                outfile << "\n";
                break;
            } case 'f': {
                infile >> arg;
                int search_result = tree.find(arg);
                outfile << arg << ": ";
                if (search_result) {
                    outfile << "found";
                } else {
                    outfile << "not found";
                }
                outfile << "\n";
                break;
            } case 'd': {
                infile >> arg;
                outfile << arg << ": ";

                bool result = tree.remove(arg);
                if(result)
                    outfile << "deleted";
                else
                    outfile << "not deleted";

                outfile << "\n";
                break;
            } case 'l': {
                outfile << "List of elements:\n";
                bst::BST<int>::Applicator f = [&](const int& val) {
                    outfile << val << "\n";
                };
                tree.traverse(bst::traverse_type::INORDER, f);
                break;
            } default: {
                std::cout << "unknown command: " << command << std::endl;
                return -1;
            }
        }
    }

    std::ofstream fout("graph.dot");
    tree.gen_dotfile(fout);
    fout.close();

    system("dot graph.dot -Tsvg -o graph.svg");
    return 0;
}
