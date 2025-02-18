#include <catch2/catch_all.hpp>

#include <algorithm>
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

using namespace std::string_literals;

// Given an array of integers representing the height of fences
// and a paint brush able to make horizontal and vertical strokes,
// find the minimum number of strokes to paint the fence.
struct Solution {
    int paint_the_fence(std::vector<int>& fence)
    {
        return paint(fence, 0, fence.size());
    }

    int paint(std::vector<int>& fence, int left, int right)
    {
        if (left >= right) {
            return 0;
        }

        int left_val = fence[left];
        int right_val = fence[right - 1];

        int width = right - left;

        if (width >= std::max(left_val, right_val)) {
            auto min_pos = left;
            for (int i = left; i < right; i++) {
                if (fence[i] < fence[min_pos]) {
                    min_pos = i;
                }
            }

            int min_val = fence[min_pos];
            for (int i = left; i < right; i++) {
                fence[i] -= min_val;
            }

            return min_val + paint(fence, left, min_pos) + paint(fence, min_pos + 1, right);
        }

        if (left_val > right_val) {
            return 1 + paint(fence, left + 1, right);
        } else {
            return 1 + paint(fence, left, right - 1);
        }
    }
};

TEST_CASE("paint_the_fence")
{
    SECTION("case 1")
    {
        auto fence = std::vector{5, 5, 1, 5, 5, 4, 1};
        int expected = 6;

        CAPTURE(fence);

        auto actual = Solution{}.paint_the_fence(fence);

        REQUIRE(actual == expected);
    }

    SECTION("case 2")
    {
        auto fence = std::vector{7, 7, 7};
        int expected = 3;

        CAPTURE(fence);

        auto actual = Solution{}.paint_the_fence(fence);

        REQUIRE(actual == expected);
    }

    SECTION("case 3")
    {
        auto fence = std::vector{7, 7, 7, 7, 7, 7, 7, 7};
        int expected = 7;

        CAPTURE(fence);

        auto actual = Solution{}.paint_the_fence(fence);

        REQUIRE(actual == expected);
    }

    SECTION("case 4")
    {
        auto fence = std::vector{0, 100, 0, 100, 0, 100, 0, 100};
        int expected = 4;

        CAPTURE(fence);

        auto actual = Solution{}.paint_the_fence(fence);

        REQUIRE(actual == expected);
    }
}
