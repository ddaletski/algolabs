#include <chrono>
#include <iostream>
#include <string>

using namespace std::string_literals;

#include <opencv2/opencv.hpp>
#include "uniform_sampler.hpp"

int main(int argc, char* argv[])
{
    if (argc > 1 && argv[1] == "visualize"s) {
        auto W = 1920;
        auto H = 1080;

        cv::Mat canvas = cv::Mat::zeros({W, H}, CV_8UC1);
        auto sampler = Sampler(canvas.size(), 20);

        for (auto sample = sampler.sample(); sample.has_value(); sample = sampler.sample()) {
            cv::circle(canvas, sample.value(), 3, {255}, -1);
            cv::imshow("main", canvas);
            cv::waitKey(1);
        }
        cv::waitKey();
    } else {
        auto W = 4096;
        auto H = 4096;

        cv::Mat canvas = cv::Mat::zeros({W, H}, CV_8UC1);
        auto sampler = Sampler(canvas.size(), 20);

        auto count = 0;
        auto start = std::chrono::high_resolution_clock::now();
        for (auto sample = sampler.sample(); sample.has_value(); sample = sampler.sample()) {
            ++count;
        }
        auto stop = std::chrono::high_resolution_clock::now();
        auto spent = std::chrono::duration_cast<std::chrono::milliseconds>(stop - start).count();
        std::cout << "took: " << spent << "ms." << std::endl;
        std::cout << "points: " << count << std::endl;
    }

    return 0;
}