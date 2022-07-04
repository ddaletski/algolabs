#include <iostream>

#include "uniform_sampler.hpp"
#include <opencv2/opencv.hpp>

int main(int argc, char* argv[]) {
    cv::Mat canvas = cv::Mat::zeros({640, 640}, CV_8UC1);
    auto sampler = Sampler(canvas.size(), 20);

    std::cout << "started" << std::endl;
    for (auto sample = sampler.sample(); sample.has_value(); sample = sampler.sample())
    {
        std::cout << "got sample: " << sample.value() << std::endl;
        cv::circle(canvas, sample.value(), 3, {255}, -1);
        cv::imshow("main", canvas);
        cv::waitKey(20);
    }
    std::cout << "finished" << std::endl;

    return 0;
}