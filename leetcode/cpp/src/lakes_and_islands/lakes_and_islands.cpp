#include <algorithm>
#include <catch2/catch_all.hpp>
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

using Point = std::array<int, 2>;
using Map = std::vector<std::vector<int>>;

template <>
struct std::hash<Point> {
    size_t operator()(const Point& p) const noexcept
    {
        auto& uarr = reinterpret_cast<const std::array<uint32_t, 2>&>(p);
        return size_t(uarr[0]) << 32 | uarr[1];
    }
};

class Solution {
public:
    Solution(Map map) : map(std::move(map))
    {
    }

    int lakes_inside_land(Point land_point)
    {
        assert(map_at(land_point) == 1);

        // mark current island as 4
        flood_fill(land_point, 4);

        int count = 0;

        for (int y = 0; y < height(); y++) {
            for (int x = 0; x < width(); x++) {
                if (map_at({y, x}) != 0) {
                    continue;
                }

                if (!touches_value({y, x}, 1, 7)) {
                    count++;
                }
            }
        }

        return count;
    }

    const Map& get_map() const
    {
        return map;
    }

private:
    void flood_fill(Point start_point, int replacement)
    {
        int orig_value = map_at(start_point);

        auto q = std::queue<Point>{};
        q.push(start_point);
        while (!q.empty()) {
            auto p = q.front();
            q.pop();

            map_at(p) = replacement;

            for (auto neighbor : neighbors_of(p)) {
                if (map_at(neighbor) == orig_value) {
                    q.push(neighbor);
                }
            }
        }
    }

    bool touches_value(Point start_point, int value, int replacement)
    {
        bool result = false;

        int orig_value = map_at(start_point);

        auto s = std::stack<Point>{};
        s.push(start_point);
        while (!s.empty()) {
            auto p = s.top();
            s.pop();

            map_at(p) = replacement;

            for (auto neighbor : neighbors_of(p)) {
                int neighbor_val = map_at(neighbor);

                if (neighbor_val == orig_value) {
                    s.push(neighbor);
                } else if (neighbor_val == value) {
                    result = true;
                    break;
                }
            }
        }

        return result;
    }

    int& map_at(Point p)
    {
        return map[p[0]][p[1]];
    }

    std::vector<Point> neighbors_of(Point p)
    {
        auto [y, x] = p;
        auto diffs = std::array<Point, 4>{Point{0, -1}, {0, 1}, {-1, 0}, {1, 0}};

        auto result = std::vector<Point>{};
        for (auto [dy, dx] : diffs) {
            auto neighbor = Point{y + dy, x + dx};
            if (neighbor[0] < 0 || neighbor[0] >= height() || neighbor[1] < 0 || neighbor[1] >= width()) {
                continue;
            }

            result.push_back(neighbor);
        }

        return result;
    }

    inline int height() const
    {
        return map.size();
    }

    inline int width() const
    {
        return map[0].size();
    }

    Map map;
};

std::ostream& operator<<(std::ostream& str, const Map& map)
{
    str << "[\n";
    for (auto row : map) {
        str << "  [ ";
        for (auto x : row) {
            str << x << " ";
        }
        str << "]\n";
    }
    str << "]\n";
    return str;
}

TEST_CASE("lakes_and_islands")
{
    auto grid = std::vector<std::vector<int>>{
        {0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},  //
        {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},  //
        {0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0},  //
        {0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},  //
        {0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0},  //
        {0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0},  //
        {0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0},  //
        {0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0}   //
    };

    SECTION("count_lakes")
    {
        auto start_point = Point{4, 4};
        int expected = 2;

        auto sol = Solution(grid);
        int actual = sol.lakes_inside_land(start_point);
        REQUIRE(actual == expected);
    }
}
