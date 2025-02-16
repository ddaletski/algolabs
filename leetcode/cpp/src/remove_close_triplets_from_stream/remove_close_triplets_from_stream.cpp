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

using Triple = std::array<int, 3>;

struct Solution {
    Solution(int d) : D(d)
    {
    }

    std::optional<Triple> remove_triple(int new_val)
    {
        auto low = values.lower_bound(new_val - D);
        auto high = values.upper_bound(new_val + D);
        auto candidates = std::vector<int>{low, high};  // at most 4 values here

        if (candidates.size() < 2) {
            values.insert(new_val);
            return {};
        }

        for (int i = 0; i < candidates.size() - 1; i++) {
            int a = candidates[i];
            int b = candidates[i + 1];
            if (b - a > D) {
                continue;
            }

            if (abs(new_val - a) <= D && abs(b - new_val) <= D) {
                values.extract(a);
                values.extract(b);

                auto found = Triple{a, b, new_val};
                std::sort(found.begin(), found.end());

                return found;
            }
        }

        return {};
    }

    const int D;
    std::multiset<int> values;
};

TEST_CASE("remove_close_triplets_from_stream")
{
    SECTION("case 1")
    {
        auto sol = Solution(2);
        REQUIRE(sol.remove_triple(1) == std::nullopt);
        REQUIRE(sol.remove_triple(2) == std::nullopt);
        REQUIRE(sol.remove_triple(3).value() == Triple{1, 2, 3});
        // empty
        REQUIRE(sol.remove_triple(3) == std::nullopt);
        REQUIRE(sol.remove_triple(2) == std::nullopt);
        REQUIRE(sol.remove_triple(7) == std::nullopt);
        REQUIRE(sol.remove_triple(8) == std::nullopt);
        REQUIRE(sol.remove_triple(3).value() == Triple{2, 3, 3});
        // 7, 8
        REQUIRE(sol.remove_triple(2) == std::nullopt);
        REQUIRE(sol.remove_triple(6).value() == Triple{6, 7, 8});
    }
}
