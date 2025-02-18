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

// Given a list of 12 tiles, each with a color and a number. Find if 4 winning hands exist.
// A winning hand consists of 3 tiles,
// and has either "all colours and number are the same" or "color is the same but number are consecutive".
// A tile can't be used in more than one hand at once

using Tile = std::pair<char, int>;
using Tiles = std::array<Tile, 12>;
using Hand = std::vector<Tile>;

template <>
struct std::hash<Tile> {
    size_t operator()(const Tile& t) const noexcept
    {
        return size_t(t.first) << 32 | t.second;
    }
};

template <typename T>
class Combinations {
public:
    Combinations(std::vector<T> source, size_t k) : _source(std::move(source)), _k(k)
    {
        stack.push({{}, _source.size()});
    }

    std::optional<std::vector<T>> next()
    {
        while (!stack.empty()) {
            auto [combination, src_bound] = stack.top();
            stack.pop();

            if (combination.size() == _k) {
                return combination;
            }

            if (combination.size() + src_bound < _k) {
                continue;
            }

            stack.push({combination, src_bound - 1});
            combination.push_back(_source[src_bound - 1]);
            stack.push({combination, src_bound - 1});
        }

        return std::nullopt;
    }

    std::vector<T> remaining()
    {
        auto res = std::vector<T>{};

        auto v = next();
        while (v) {
            res.push_back(v.value());
            v = next();
        }

        return res;
    }

private:
    std::stack<std::pair<std::vector<T>, size_t>> stack;
    const std::vector<T> _source;
    size_t _k;
};

struct Solution {
    std::vector<Hand> winning_hands(const Tiles& tiles)
    {
        auto full_hands = std::vector<Hand>{};
        auto tiles_left = std::unordered_multiset<Tile>{tiles.begin(), tiles.end()};

        if (backtrack(full_hands, tiles_left)) {
            return full_hands;
        }

        return {};
    }

    bool backtrack(std::vector<Hand>& full_hands, std::unordered_multiset<Tile>& tiles_left)
    {
        if (full_hands.size() == 4) {
            return true;
        } else if (tiles_left.size() < 3) {
            return false;
        }

        auto all_tiles = std::vector(tiles_left.begin(), tiles_left.end());
        auto combinator = Combinations(all_tiles, 3);

        while (true) {
            auto next = combinator.next();
            if (!next) {
                break;
            }
            auto combination = next.value();

            if (!valid_hand(combination)) {
                continue;
            }

            for (auto tile : combination) {
                tiles_left.extract(tile);
            }
            full_hands.push_back(combination);

            if (backtrack(full_hands, tiles_left)) {
                return true;
            }

            for (auto tile : combination) {
                tiles_left.insert(tile);
            }
            full_hands.pop_back();
        }

        return false;
    }

    bool valid_hand(const Hand& hand)
    {
        assert(hand.size() == 3);

        auto colors = std::unordered_set<char>{};
        auto values = std::vector<int>{};

        for (auto& [color, value] : hand) {
            colors.insert(color);
            values.push_back(value);
        }

        if (colors.size() > 1) {
            return false;
        }

        std::sort(values.begin(), values.end());
        if (values.front() == values.back()) {
            return true;
        }

        for (int i = 1; i < values.size(); i++) {
            if (values[i] != values[i - 1] + 1) {
                return false;
            }
        }

        return true;
    }
};

TEST_CASE("winning_hands")
{
    SECTION("case 1")
    {
        auto tiles = Tiles{
            Tile{'R', 1}, {'R', 2}, {'R', 3},  //
            {'B', 5},     {'B', 5}, {'B', 5},  //
            {'G', 7},     {'G', 8}, {'G', 9},  //
            {'Y', 1},     {'Y', 2}, {'Y', 3},  //
        };
        std::shuffle(tiles.begin(), tiles.end(), std::minstd_rand0(0));

        REQUIRE(Solution{}.winning_hands(tiles).size() == 4);
    }

    SECTION("case 2")
    {
        auto tiles = Tiles{
            Tile{'R', 1}, {'R', 2}, {'R', 3},  //
            {'B', 5},     {'B', 5}, {'B', 5},  //
            {'G', 7},     {'G', 8}, {'G', 9},  //
            {'Y', 1},     {'Y', 2}, {'Y', 4},  //
        };
        std::shuffle(tiles.begin(), tiles.end(), std::minstd_rand0(0));

        REQUIRE(Solution{}.winning_hands(tiles).size() == 0);
    }
}
