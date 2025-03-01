#include <catch2/catch_all.hpp>

#include <algorithm>
#include <deque>
#include <functional>
#include <iostream>
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

// you can see how well the algorithm behaved using these metrics
namespace {
size_t brute_checks = 0;
size_t total_checks = 0;
size_t hash_collisions = 0;
}  // namespace

struct Solution {
    std::vector<std::string> mergeScreenshots(std::vector<std::string>& screenshot1,
                                              std::vector<std::string>& screenshot2)
    {
        auto lines_to_skip = redundant_lines(screenshot1, screenshot2);

        std::copy(screenshot2.begin() + lines_to_skip, screenshot2.end(), std::back_inserter(screenshot1));

        return screenshot1;
    }

    // return the number of lines to skip in s2
    size_t redundant_lines(const std::vector<std::string>& s1, const std::vector<std::string>& s2)
    {
        // some really big number, but not too big to avoid overflow
        // the rolling hash computation inside the loops set the upper bound:
        // for any (a, b, c) < modulo, (a * b + c) should not overflow uint64_t
        const size_t MODULO = UINT32_MAX / 2 - 1;
        // polynomial base (prime)
        const size_t BASE = 31;

        size_t rolling1 = 0;
        size_t rolling2 = 0;
        size_t base_pow = 1;

        const size_t n = s1.size();
        const size_t m = s2.size();

        auto candidates = std::stack<size_t>{};

        for (size_t i = 0; i < std::min(n, m); i++) {
            // 32 bit int is important here to avoid overflow
            // so we strip the upper 32 bits of the hash
            uint32_t line_hash1 = std::hash<std::string>{}(s1[n - i - 1]);
            uint32_t line_hash2 = std::hash<std::string>{}(s2[i]);

            rolling1 = (rolling1 + base_pow * line_hash1) % MODULO;
            rolling2 = (rolling2 * BASE + line_hash2) % MODULO;
            base_pow = (base_pow * BASE) % MODULO;

            total_checks++;

            if (rolling1 == rolling2) {
                candidates.push(i + 1);
            }
        }

        // check the longest matches first
        while (!candidates.empty()) {
            size_t overlap = candidates.top();
            candidates.pop();

            brute_checks++;
            if (!check_match(s1, s2, overlap)) {
                hash_collisions++;
                continue;
            }
            return overlap;
        }

        return 0;
    }

    // check if the last n_lines in s1 is the same as the first n_lines in s2
    bool check_match(const std::vector<std::string>& s1, const std::vector<std::string>& s2, size_t n_lines)
    {
        const size_t n = s1.size();
        for (size_t i = 0; i < n_lines; i++) {
            if (s1[n - n_lines + i] != s2[i]) {
                return false;
            }
        }

        return true;
    }
};

auto rng = std::default_random_engine{};

std::string gen_line(size_t width, char min_char = 'A', char max_char = 'Z')
{
    auto dist = std::uniform_int_distribution<int>(min_char, max_char);

    char c = dist(rng);

    return std::string(width, c);
}

std::tuple<std::vector<std::string>, std::vector<std::string>, std::vector<std::string>> generate_case(size_t total_len,
                                                                                                       size_t width,
                                                                                                       size_t overlap)
{
    size_t size1 = (total_len - overlap) / 2;
    size_t size2 = total_len - overlap - size1;

    auto full = std::vector<std::string>{};
    full.reserve(total_len);

    std::generate_n(std::back_inserter(full), size1, [&]() { return gen_line(width, 'A', 'F'); });
    std::generate_n(std::back_inserter(full), overlap, [&]() { return gen_line(width, 'G', 'M'); });
    std::generate_n(std::back_inserter(full), size2, [&]() { return gen_line(width, 'N', 'Z'); });

    auto s1 = std::vector<std::string>{};
    auto s2 = std::vector<std::string>{};

    std::copy(full.begin(), full.begin() + size1 + overlap, std::back_inserter(s1));
    std::copy(full.begin() + size1, full.end(), std::back_inserter(s2));

    return {s1, s2, full};
}

void check_collisions(double max_brute_ratio)
{
    double brute_check_rate = brute_checks == 0 ? 0.0 : 1.0 * brute_checks / total_checks;
    std::cout << "total checks: " << total_checks << std::endl;
    std::cout << "brute-force checks: " << brute_checks << std::endl;
    std::cout << "brute-force rate: " << brute_check_rate << std::endl;
    REQUIRE(brute_check_rate <= max_brute_ratio);

    total_checks = 0;
    brute_checks = 0;
    hash_collisions = 0;
}

TEST_CASE("merge_screenshots")
{
    SECTION("small cases")
    {
        const size_t height = 8;
        size_t overlap = GENERATE(0, 1, 2, 3, 4, 5);

        for (size_t i = 0; i < 100; i++) {
            auto [s1, s2, expected] = generate_case(height, 3, overlap);
            CAPTURE(s1, s2, expected);

            auto sol = Solution{};
            auto actual = sol.mergeScreenshots(s1, s2);

            REQUIRE(actual == expected);
        }

        check_collisions(2.0 / height);
    }

    SECTION("big cases")
    {
        const size_t height = 1000;
        for (int i = 0; i < 1000; i++) {
            size_t overlap = std::uniform_int_distribution<size_t>(0, height)(rng);

            auto [s1, s2, expected] = generate_case(height, height, overlap);

            auto sol = Solution{};
            auto actual = sol.mergeScreenshots(s1, s2);

            REQUIRE(actual == expected);
        }

        check_collisions(2.0 / height);
    }

    SECTION("huge constant overlap")
    {
        auto line = gen_line(1000, 'A', 'A');
        auto s1 = std::vector<std::string>(1000, line);
        s1.front() = gen_line(1000, 'B');
        auto s2 = std::vector<std::string>(1000, line);
        s2.back() = gen_line(1000, 'C');

        auto expected = std::vector<std::string>(1001, line);
        expected.front() = s1.front();
        expected.back() = s2.back();

        auto actual = Solution{}.mergeScreenshots(s1, s2);

        REQUIRE(actual == expected);

        check_collisions(2.0 / 1000);
    }
}
