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

struct Solution {
    int makeTriplets(std::vector<int>& A, std::vector<int>& B, std::vector<int>& C, int d)
    {
        auto arr = merge(A, B, C);
        int n = arr.size();

        auto sub_counts = std::array<int, 3>{};

        int result = 0;

        int left = 0;
        for (int right = 0; right < n; right++) {
            sub_counts[arr[right].second]++;

            while (left < right && arr[right].first - arr[left].first > d) {
                sub_counts[arr[left].second]--;
                left++;
            }

            result += sub_counts[0] * sub_counts[1] * sub_counts[2] / sub_counts[arr[right].second];
        }

        return result;
    }

    std::vector<std::pair<int, uint8_t>> merge(const std::vector<int>& A,
                                               const std::vector<int>& B,
                                               const std::vector<int>& C)
    {
        auto result = std::vector<std::pair<int, uint8_t>>(A.size() + B.size() + C.size());

        auto res_it = result.begin();
        auto a_it = A.begin();
        auto b_it = B.begin();
        auto c_it = C.begin();

        while (a_it != A.end() || b_it != B.end() || c_it != C.end()) {
            int a = a_it == A.end() ? std::numeric_limits<int>::max() : *a_it;
            int b = b_it == B.end() ? std::numeric_limits<int>::max() : *b_it;
            int c = c_it == C.end() ? std::numeric_limits<int>::max() : *c_it;

            if (a <= b && a <= c) {
                *res_it = {a, 0};
                a_it++;
            } else if (b <= a && b <= c) {
                *res_it = {b, 1};
                b_it++;
            } else {
                *res_it = {c, 2};
                c_it++;
            }

            res_it++;
        }

        return result;
    }
};

TEST_CASE("triples_with_absdiff_constraints")
{
    SECTION("case 1")
    {
        auto A = std::vector<int>{0, 1};
        auto B = std::vector<int>{0, 1};
        auto C = std::vector<int>{0, 1};
        auto d = 1;

        auto expected = 8;
        auto actual = Solution{}.makeTriplets(A, B, C, d);

        REQUIRE(actual == expected);
    }

    SECTION("case 2")
    {
        auto A = std::vector<int>{1, 2};
        auto B = std::vector<int>{1, 3};
        auto C = std::vector<int>{2, 3};
        auto d = 1;

        auto expected = 4;
        auto actual = Solution{}.makeTriplets(A, B, C, d);

        REQUIRE(actual == expected);
    }
}
