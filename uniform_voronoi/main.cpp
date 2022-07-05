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
        auto img = cv::imread("input.png");
        auto radius = sqrt(1.0 * img.size().area()) / 100;
        auto sampler = Sampler(img.size(), radius);

        std::vector<cv::Point2f> points;
        {
            auto start = std::chrono::high_resolution_clock::now();
            for (auto sample = sampler.sample(); sample.has_value(); sample = sampler.sample()) {
                points.push_back(sample.value());
            }
            auto stop = std::chrono::high_resolution_clock::now();
            auto spent = std::chrono::duration_cast<std::chrono::milliseconds>(stop - start).count();
            std::cout << "sampling took: " << spent << "ms." << std::endl;
            std::cout << "points: " << points.size() << std::endl;
        }

        {
            auto start = std::chrono::high_resolution_clock::now();
            auto subdiv = cv::Subdiv2D(cv::Rect(0, 0, img.cols, img.rows));
            subdiv.insert(points);

            std::vector<std::vector<cv::Point2f>> facets;
            std::vector<cv::Point2f> facetCenters;
            subdiv.getVoronoiFacetList({}, facets, facetCenters);

            for (auto i = 0; i < facets.size(); ++i) {
                auto& facet = facets[i];
                cv::Point center = facetCenters[i];

                std::vector<cv::Point> polygon;
                polygon.reserve(facet.size());
                for (auto& p : facet) {
                    polygon.push_back(p);
                }
                auto colorRaw = img.at<cv::Vec3b>(center);
                cv::Scalar color = colorRaw;
                cv::fillConvexPoly(img, polygon, color);
            }

            auto stop = std::chrono::high_resolution_clock::now();
            auto spent = std::chrono::duration_cast<std::chrono::milliseconds>(stop - start).count();
            std::cout << "render took: " << spent << "ms." << std::endl;

            cv::imwrite("out.png", img);
        }
    }

    return 0;
}