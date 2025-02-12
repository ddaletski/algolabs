#define TESTING
#include "avl_tree.h"
#include <catch2/catch_all.hpp>
#include <random>
#include <numeric>
#include <map>

std::vector<std::pair<int, float>> rand_dataset(int min_key, int max_key, size_t count) {
    auto rng = std::default_random_engine(0);

    auto key_dist = std::uniform_int_distribution(min_key, max_key);
    auto val_dist = std::uniform_real_distribution();

    auto result = std::vector<std::pair<int, float>>{};
    result.reserve(count);

    std::generate_n(std::back_inserter(result), count, [&]() {
        int key = key_dist(rng);
        float val = val_dist(rng);

        return std::make_pair(key, val);
    });

    return result;
}

TEST_CASE( "AVL tree behavior matches std::map", "[avl]" ) {
    const int COUNT = 1000;
    const int MIN = -COUNT * 5;
    const int MAX = COUNT * 5;
    auto dataset = rand_dataset(MIN, MAX, COUNT);

    auto stdmap = std::map<int, float>{};
    auto avl = AVLTree<int, float>();

    for (auto& [k, v] : dataset) {
        stdmap[k] = v;
        avl.insert(k, v);

        CAPTURE(k, avl.size());
        REQUIRE(avl.is_balanced());
    }

    REQUIRE(avl.size() == stdmap.size());
    REQUIRE(avl.is_balanced());

    SECTION("find matches after insertion") {
        for (int i = MIN; i <= MAX; i++) {
            CAPTURE(i);
            auto it = stdmap.find(i);
            auto opt = avl.find(i);

            if (it != stdmap.end()) {
                REQUIRE(opt.has_value());
                REQUIRE(opt.value() == it->second);
            } else {
                REQUIRE(!opt.has_value());
            }
        }
    }

    for (auto& [k, v] : dataset) {
        if (rand() % 2) {
            stdmap.erase(k);
            avl.remove(k);
        }
    }

    REQUIRE(avl.size() == stdmap.size());
    REQUIRE(avl.is_balanced());

    SECTION("find matches after deletion") {
        for (int i = MIN; i <= MAX; i++) {
            CAPTURE(i);
            auto it = stdmap.find(i);
            auto opt = avl.find(i);

            if (it != stdmap.end()) {
                REQUIRE(opt.has_value());
                REQUIRE(opt.value() == it->second);
            } else {
                INFO(i << " was removed");
                REQUIRE(!opt.has_value());
            }
        }
    }
}