#pragma once

#include <vector>
#include <queue>
#include <optional>

#include <opencv2/opencv.hpp>

class Sampler {
public:
    Sampler(cv::Size size, float radius)
    : _radius(radius)
    , _radius2(radius * radius)
    , _cellSize(radius * sqrt(0.5f))
    , _extent({0, 0}, size)
    {
        _gridSize = cv::Size(ceil(size.width / _cellSize), ceil(size.height / _cellSize));
        _gridValues.resize(_gridSize.area());
        _gridTaken.resize(_gridSize.area(), false);

        addSample(size / 2);
    }

    std::optional<cv::Point> sample() {
        while (!_queue.empty()) {
            size_t parentIdx = rand() % _queue.size();
            const auto parent = _queue[parentIdx];
            float seed = 1.0f * rand() / RAND_MAX;
            float epsilon = 0.0000001f;

            // Make a new candidate.
            for (auto j = 0; j < K; ++j) {
                float angle = 2 * M_PI * (seed + 1.0f*j/K);
                float r = _radius + epsilon;
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
    const int K = 4;

    std::vector<cv::Point> _gridValues;
    std::vector<bool> _gridTaken;
    std::vector<cv::Point> _queue;

    float _cellSize;
    cv::Size _gridSize;
    cv::Rect _extent;
    float _radius;
    float _radius2;

    void addSample(cv::Point sample) {
        size_t gridPos = _gridSize.width * int(sample.y / _cellSize) + int(sample.x / _cellSize);

        _gridTaken[gridPos] = true;
        _gridValues[gridPos] = sample;
        _queue.push_back(sample);
    }

    bool farFromOtherPoints(cv::Point point) {
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
                    int dx = cellValue.x - point.x;
                    int dy = cellValue.y - point.y;
                    if (dx * dx + dy * dy < _radius2) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
};
/*

function* poissonDiscSampler(width, height, radius) {
  const k = 4; // maximum number of samples before rejection
  const radius2 = radius * radius;
  const cellSize = radius * Math.SQRT1_2;
  const gridWidth = Math.ceil(width / cellSize);
  const gridHeight = Math.ceil(height / cellSize);
  const grid = new Array(gridWidth * gridHeight);
  const queue = [];

  // Pick the first sample.
  yield {add: sample(width / 2 , height / 2, null)};

  // Pick a random existing sample from the queue.
  pick: while (queue.length) {
    const i = Math.random() * queue.length | 0;
    const parent = queue[i];
    const seed = Math.random();
    const epsilon = 0.0000001;

    // Make a new candidate.
    for (let j = 0; j < k; ++j) {
      const a = 2 * Math.PI * (seed + 1.0*j/k);
      const r = radius + epsilon;
      const x = parent[0] + r * Math.cos(a);
      const y = parent[1] + r * Math.sin(a);

      // Accept candidates that are inside the allowed extent
      // and farther than 2 * radius to all existing samples.
      if (0 <= x && x < width && 0 <= y && y < height && far(x, y)) {
        yield {add: sample(x, y), parent};
        continue pick;
      }
    }

    // If none of k candidates were accepted, remove it from the queue.
    const r = queue.pop();
    if (i < queue.length) queue[i] = r;
    yield {remove: parent};
  }

  function far(x, y) {
    const i = x / cellSize | 0;
    const j = y / cellSize | 0;
    const i0 = Math.max(i - 2, 0);
    const j0 = Math.max(j - 2, 0);
    const i1 = Math.min(i + 3, gridWidth);
    const j1 = Math.min(j + 3, gridHeight);
    for (let j = j0; j < j1; ++j) {
      const o = j * gridWidth;
      for (let i = i0; i < i1; ++i) {
        const s = grid[o + i];
        if (s) {
          const dx = s[0] - x;
          const dy = s[1] - y;
          if (dx * dx + dy * dy < radius2) return false;
        }
      }
    }
    return true;
  }

  function sample(x, y, parent) {
    const s = grid[gridWidth * (y / cellSize | 0) + (x / cellSize | 0)] = [x, y];
    queue.push(s);
    return s;
  }
}
*/