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

using City = std::string;
using Edge = std::pair<City, City>;

struct Solution {
    Solution(const std::vector<Edge>& connections)
    {
        for (const auto& [left, right] : connections) {
            _adj_list[left].insert(right);
            _adj_list[right].insert(left);
            _adj_list["$$$"].insert(left);  // dummy city connected to all others
            _adj_list["$$$"].insert(right);
        }
    }

    // note: this solution returns a correct itinerary of unique cities
    std::vector<City> correct_itinerary(const std::vector<City>& itinerary)
    {
        auto best_path = std::vector<City>{};
        int best_distance = INT_MAX;
        auto visited = std::unordered_set<City>{};

        auto path = std::vector<City>{"$$$"};
        trace_path(itinerary, path, 0, best_path, best_distance, visited);

        return best_path;
    }

    void trace_path(const std::vector<City>& itinerary,
                    std::vector<City>& path,
                    int current_distance,
                    std::vector<City>& best_path,
                    int best_distance,
                    std::unordered_set<City>& visited)
    {
        if (current_distance > best_distance) {
            return;
        }

        if (path.size() == itinerary.size() + 1) {
            if (current_distance < best_distance) {
                best_distance = current_distance;
                best_path.clear();
                best_path.insert(best_path.begin(), path.begin() + 1, path.end());  // skip first $$$
            }
            return;
        }

        auto last_city = path.back();
        visited.insert(last_city);

        for (auto& n : _adj_list[last_city]) {
            if (visited.count(n)) {
                continue;
            }
            path.push_back(n);
            trace_path(itinerary, path, current_distance + difference(last_city, n), best_path, best_distance, visited);
            path.pop_back();
        }

        visited.erase(last_city);
    }

    int difference(const City& a, const City& b)
    {
        int res = 0;
        for (int i = 0; i < 3; i++) {
            res += a[i] != b[i];
        }
        return res;
    }

private:
    std::unordered_map<City, std::unordered_set<City>> _adj_list;
};

struct Solution2 {
    Solution2(const std::vector<Edge>& connections)
    {
        auto city_to_id = std::unordered_map<std::string, int>{};

        int id = 0;
        for (const auto& [left, right] : connections) {
            if (!city_to_id.count(left)) {
                city_to_id[left] = id++;
            }
            if (!city_to_id.count(right)) {
                city_to_id[right] = id++;
            }

            int left_id = city_to_id[left];
            int right_id = city_to_id[right];

            _adj_list[left_id].push_back(right_id);
            _adj_list[right_id].push_back(left_id);
        }

        _cities.resize(city_to_id.size());
        for (auto& [city, id] : city_to_id) {
            _cities[id] = city;
        }
    }

    // note: this solution can return an itinerary, containing a particular city multiple times
    std::vector<City> correct_itinerary(const std::vector<City>& itinerary)
    {
        const int N = itinerary.size();
        const int M = _cities.size();

        auto dp = std::vector<std::vector<int>>(N, std::vector<int>(M, INT_MAX));
        auto prev = std::vector<std::vector<int>>(N, std::vector<int>(M, -1));

        for (int city = 0; city < M; city++) {
            dp[0][city] = difference(_cities[city], itinerary[0]);
        }

        for (int pos = 1; pos < N; pos++) {
            for (int city = 0; city < M; city++) {
                int chars_diff = difference(_cities[city], itinerary[pos]);

                int min_dist = INT_MAX;
                for (auto neighbor : _adj_list[city]) {
                    int dist = dp[pos - 1][neighbor] + chars_diff;
                    if (dist < min_dist) {
                        prev[pos][city] = neighbor;
                        min_dist = dist;
                    }
                }
                dp[pos][city] = min_dist;
            }
        }

        auto last_city = 0;
        for (int city = 1; city < M; city++) {
            if (dp.back()[city] < dp.back()[last_city]) {
                last_city = city;
            }
        }

        auto path = std::vector<City>{};
        for (int i = N - 1; i >= 0; --i) {
            path.push_back(_cities[last_city]);
            last_city = prev[i][last_city];
        }
        std::reverse(path.begin(), path.end());

        return path;
    }

    int difference(const City& a, const City& b)
    {
        int res = 0;
        for (int i = 0; i < 3; i++) {
            res += a[i] != b[i];
        }
        return res;
    }

private:
    std::vector<City> _cities;
    std::unordered_map<int, std::vector<int>> _adj_list;
};

TEST_CASE("correct_itinerary")
{
    SECTION("case 1")
    {
        auto connections = std::vector<Edge>{{"AAA", "BBX"}, {"BBX", "CCC"}, {"CCC", "DAD"}, {"CCC", "XXD"}};
        auto itinerary = std::vector<City>{"AAA", "BBB", "CCC", "DDD"};
        auto expected = std::vector<City>{"AAA", "BBX", "CCC", "DAD"};

        auto actual = Solution2{connections}.correct_itinerary(itinerary);
        REQUIRE(actual == expected);
    }

    SECTION("case 2")
    {
        auto connections = std::vector<Edge>{{"AAA", "BBB"}, {"BBB", "CCC"}, {"CCC", "DDD"}, {"BBB", "CAC"}};
        auto itinerary = std::vector<City>{"AAA", "BBB", "CCC", "DDD"};
        auto expected = std::vector<City>{"AAA", "BBB", "CCC", "DDD"};

        auto actual = Solution2{connections}.correct_itinerary(itinerary);
        REQUIRE(actual == expected);
    }

    SECTION("case 2")
    {
        auto connections = std::vector<Edge>{{"AAA", "XXX"}, {"XXX", "YYY"}, {"YYY", "DDZ"}};
        auto itinerary = std::vector<City>{"AAA", "BBB", "CCC", "DDD"};
        auto expected = std::vector<City>{"AAA", "XXX", "YYY", "DDZ"};

        auto actual = Solution2{connections}.correct_itinerary(itinerary);
        REQUIRE(actual == expected);
    }
}
