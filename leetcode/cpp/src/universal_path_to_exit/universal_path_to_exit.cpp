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

using namespace std::string_literals;

namespace {

template <typename T>
using Mat = std::vector<std::vector<T>>;

using Point = std::array<int, 2>;
using Vec2 = Point;

const int EMPTY = 0;
const int WALL = 1;
const int EXIT = 2;

}  // namespace

template <>
struct std::hash<Point> {
    size_t operator()(const Point& p) const noexcept
    {
        auto p_unigned = reinterpret_cast<const std::array<uint32_t, 2>&>(p);

        return size_t(p_unigned[0]) << 32 | p_unigned[1];
    }
};

// You are given a matrix of 0s, 1s, and 2s, where 0 means an empty cell, 1 means a wall, and 2 means an exit.
// There will always be an exit.
// Generate a sequence of instructions (U, D, L, R)
// such that no matter where you start in the matrix, you always reach the exit.
class Solution {
public:
    Solution(const Mat<int>& grid) : grid(grid), width(grid[0].size()), height(grid.size())
    {
    }

    std::string universal_path()
    {
        auto step_map = find_all_shortest_paths();

        // open cells for which we still didn't find an exit
        auto start_candidates = std::unordered_set<Point>{};
        // current position for each open cell after moving along the cocatenated path
        auto curr_pos = std::unordered_map<Point, Point>{};

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                auto p = Point{y, x};
                if (grid[y][x] == EMPTY) {
                    start_candidates.insert(p);
                    curr_pos[p] = p;
                }
            }
        }

        auto result = ""s;

        while (!start_candidates.empty()) {
            auto start = *start_candidates.begin();
            start_candidates.erase(start_candidates.begin());

            // find the shortest path to an exit from a particular start point
            auto path = trace_shortest_path(curr_pos[start], step_map);

            // apply path moves to all other unsolved starting points
            for (char d : path) {
                auto dir_vec = parse_direction(d);

                for (auto it = start_candidates.begin(); it != start_candidates.end();) {
                    auto candidate = *it;

                    auto& last_pos = curr_pos[candidate];
                    last_pos = move(last_pos, dir_vec);

                    if (grid[last_pos[0]][last_pos[1]] == EXIT) {
                        // exit reached for a candidate, remove the candidate
                        it = start_candidates.erase(it);
                    } else {
                        it++;
                    }
                }
            }

            result += path;
        }

        return result;
    }

    // validate solution automatically
    bool validate_path(const std::string& path)
    {
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (grid[y][x] != EMPTY) {
                    continue;
                }

                bool found_exit = false;

                auto pos = Point{y, x};
                for (char d : path) {
                    auto dir_vec = parse_direction(d);
                    pos = move(pos, dir_vec);

                    if (grid[pos[0]][pos[1]] == EXIT) {
                        found_exit = true;
                        break;
                    }
                }

                if (!found_exit) {
                    return false;
                }
            }
        }
        return true;
    }

private:
    // find shortest path from all open cells to the closest exit
    // returns a matrix where M[y][x] is the next point along the shortest path towards an exit
    Mat<Point> find_all_shortest_paths()
    {
        auto prev_map = Mat<Point>(height, std::vector<Point>(width));
        auto min_dist = Mat<int>(height, std::vector<int>(width, INT_MAX));

        auto q = std::queue<std::pair<Point, int>>{};

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                prev_map[y][x] = {y, x};

                if (grid[y][x] != EXIT) {
                    continue;
                }

                q.push({{y, x}, 0});
                min_dist[y][x] = 0;
            }
        }

        while (!q.empty()) {
            auto [p, dist] = q.front();
            auto [y, x] = p;

            q.pop();

            for (char d : "LURD"s) {
                auto dir_vec = parse_direction(d);
                auto neighbor = move(p, dir_vec);
                if (neighbor == p) {
                    continue;
                }

                auto [ny, nx] = neighbor;

                if (dist + 1 >= min_dist[ny][nx]) {
                    continue;
                }
                min_dist[ny][nx] = dist + 1;
                prev_map[ny][nx] = p;

                q.push({neighbor, dist + 1});
            }
        }

        return prev_map;
    }

    // trace shortest path from a particular point to the closest exit
    std::string trace_shortest_path(Point start, const Mat<Point>& step_mat)
    {
        auto path = ""s;

        auto curr = start;
        auto next = step_mat[curr[0]][curr[1]];

        while (next != curr) {
            char d = dump_direction(diff(next, curr));
            path.push_back(d);

            curr = next;
            next = step_mat[curr[0]][curr[1]];
        }

        return path;
    }

    Vec2 diff(Point next, Point prev)
    {
        int dy = next[0] - prev[0];
        int dx = next[1] - prev[1];
        return {dy, dx};
    }

    char dump_direction(Vec2 dir_vec)
    {
        auto [dy, dx] = dir_vec;

        if (dy == 1) {
            return 'D';
        } else if (dy == -1) {
            return 'U';
        } else if (dx == -1) {
            return 'L';
        } else {
            return 'R';
        }
    }

    // move from point `p` in direction `d` if possible
    Point move(Point p, Vec2 d)
    {
        auto [py, px] = p;
        auto [dy, dx] = d;
        int y = py + dy;
        int x = px + dx;

        if (y < 0 || y >= height || x < 0 || x >= width) {
            return p;
        }

        if (grid[y][x] == WALL) {
            return p;
        }

        return {y, x};
    }

    Vec2 parse_direction(char d)
    {
        int dy = 0;
        int dx = 0;

        switch (d) {
            case 'L':
                dx = -1;
                break;
            case 'R':
                dx = 1;
                break;
            case 'U':
                dy = -1;
                break;
            case 'D':
                dy = 1;
                break;
            default:
                throw std::runtime_error("invalid direction: '"s + d + "'");
        }

        return {dy, dx};
    }

    const Mat<int> grid;
    const int width;
    const int height;
};

TEST_CASE("universal_path_to_exit")
{
    SECTION("case 1")
    {
        auto grid = Mat<int>{
            {1, 0, 0},  //
            {0, 2, 0},  //
            {1, 0, 1},  //
        };

        auto sol = Solution(grid);

        auto seq = sol.universal_path();
        REQUIRE(sol.validate_path(seq));
    }

    SECTION("case 2")
    {
        int N = GENERATE(10, 30, 100);
        auto grid = Mat<int>(N, std::vector<int>(N, 0));
        for (int i = 0; i < std::max(N / 10, 1); i++) {
            int x = rand() % N;
            int y = rand() % N;

            grid[y][x] = 2;
        }

        auto sol = Solution(grid);

        auto seq = sol.universal_path();
        REQUIRE(sol.validate_path(seq));
    }
}
