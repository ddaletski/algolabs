#include <algorithm>
#include <catch2/catch_all.hpp>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <random>
#include <set>
#include <stack>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// find longest subarray such that it's sum + initial_val is non-negative
struct Solution {
    std::vector<int> longest_nonnegative_window(const std::vector<int>& values, int initial_value = 0)
    {
        int best_start = 0;
        int max_len = 0;

        int start = 0;
        for (int end = 0; end < values.size(); end++) {
            initial_value += values[end];
            if (initial_value >= 0 && end - start + 1 > max_len) {
                max_len = end - start + 1;
                best_start = start;
            }

            while (initial_value < 0) {
                initial_value -= values[start];
                start++;
            }
        }

        return std::vector<int>(values.begin() + best_start, values.begin() + best_start + max_len);
    }
};

TEST_CASE("longest_nonnegative_window")
{
    SECTION("case1")
    {
        auto values = std::vector<int>{1, -3, 5, -2, 1};

        auto expected = std::vector<int>{5, -2, 1};
        auto actual = Solution{}.longest_nonnegative_window(values, 1);

        REQUIRE(actual == expected);
    }

    SECTION("case2")
    {
        auto values = std::vector<int>{1, -3, 5, -2, 1};

        auto expected = std::vector<int>{1, -3, 5, -2, 1};
        auto actual = Solution{}.longest_nonnegative_window(values, 2);

        REQUIRE(actual == expected);
    }

    SECTION("case3")
    {
        auto values = std::vector<int>{-3, -4, -5, 1, -5};

        auto expected = std::vector<int>{1};
        auto actual = Solution{}.longest_nonnegative_window(values, 2);

        REQUIRE(actual == expected);
    }

    SECTION("case4")
    {
        auto values = std::vector<int>{-3, -4, -5, -1, -5};

        auto expected = std::vector<int>{};
        auto actual = Solution{}.longest_nonnegative_window(values, 0);

        REQUIRE(actual == expected);
    }
}
