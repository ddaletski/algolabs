#pragma once

#include <optional>
#include <queue>
#include <random>
#include <vector>

#include <boost/random.hpp>
#include <opencv2/opencv.hpp>

class Sampler {
 public:
    Sampler(cv::Size size, float radius)
        : _radius(radius)
        , _radius2(radius * radius)
        , _ringThickness(radius / sqrt(19))
        , _cellSize(radius * sqrt(0.5f))
        , _extent({0, 0}, size)
    {
        _gridSize = cv::Size(ceil(size.width / _cellSize), ceil(size.height / _cellSize));
        _gridValues.resize(_gridSize.area());
        _gridTaken.resize(_gridSize.area(), false);

        addSample(size / 2);
    }

    std::optional<cv::Point> sample()
    {
        while (!_queue.empty()) {
            size_t parentIdx = _rand() % _queue.size();
            const auto parent = _queue[parentIdx];

            float seed = 1.0 * _rand() / RAND_MAX;

            // Make a new candidate.
            for (auto j = 0; j < K; ++j) {
                auto r = _radius + 1.0 * _ringThickness * _rand() / RAND_MAX;

                float angle = 2 * M_PI * (seed + 1.0f * j / K);

                int x = parent.x + r * cos(angle);
                int y = parent.y + r * sin(angle);
                auto candidate = cv::Point(x, y);

                // Accept candidates that are inside the allowed extent
                // and farther than 2 * radius to all existing samples.
                if (candidate.inside(_extent) && farFromOtherPoints(candidate)) {
                    addSample(candidate);
                    return candidate;
                }
            }

            // If none of k candidates were accepted, remove it from the queue.
            std::swap(_queue[parentIdx], _queue[_queue.size() - 1]);
            _queue.pop_back();
        }

        return std::nullopt;
    }

 private:
    const int K = 8;

    std::vector<cv::Point> _gridValues;
    std::vector<bool> _gridTaken;
    std::vector<cv::Point> _queue;

    float _cellSize;
    cv::Size _gridSize;
    cv::Rect _extent;
    float _radius;
    float _radius2;
    float _ringThickness;

    std::uniform_real_distribution<float> _distr01;
    boost::random::taus88 _rand;
    // std::minstd_rand0 _rand;

    void addSample(cv::Point sample)
    {
        int gridY = sample.y / _cellSize;
        int gridX = sample.x / _cellSize;
        size_t gridPos = gridY * _gridSize.width + gridX;

        _gridTaken[gridPos] = true;
        _gridValues[gridPos] = sample;
        _queue.push_back(sample);
    }

    bool farFromOtherPoints(cv::Point point)
    {
        int cellY = point.y / _cellSize;
        int cellX = point.x / _cellSize;
        int yMin = std::max(cellY - 2, 0);
        int xMin = std::max(cellX - 2, 0);
        int yMax = std::min(cellY + 3, _gridSize.height);
        int xMax = std::min(cellX + 3, _gridSize.width);

        for (auto i = yMin; i < yMax; ++i) {
            size_t offset = i * _gridSize.width;
            for (auto j = xMin; j < xMax; ++j) {
                size_t gridPos = offset + j;
                auto cellTaken = _gridTaken[gridPos];
                if (cellTaken) {
                    auto cellValue = _gridValues[gridPos];
                    auto dx = cellValue.x - point.x;
                    auto dy = cellValue.y - point.y;
                    if (dx * dx + dy * dy < _radius2) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
};