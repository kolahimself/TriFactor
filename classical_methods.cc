#include <iostream>
#include <cmath>
#include <unordered_map>
#include <string>

// Function to calculate the bearing capacity factors using Terzaghi's method
std::unordered_map<std::string, double> terzaghi(double phi) {
    std::unordered_map<std::string, double> factors;

    double exponent = exp(M_PI * (0.75 - (phi / 360.0)) * tan(phi * M_PI / 180.0));
    double numerator_q = exponent * exponent;
    double denominator_q = 2 * pow(cos((45.0 + (phi / 2.0)) * M_PI / 180.0), 2);
    double Nq = numerator_q / denominator_q;

    double Nc = (phi > 0) ? ((Nq - 1) * (1 / tan(phi * M_PI / 180.0))) : 5.71;

    double numerator_g = 2 * (Nq + 1) * tan(phi * M_PI / 180.0);
    double denominator_g = 1 + (0.4 * sin(4 * phi * M_PI / 180.0));
    double Ngamma = numerator_g / denominator_g;

    factors["Nc"] = Nc;
    factors["Nq"] = Nq;
    factors["Ngamma"] = Ngamma;

    return factors;
}

// Function to calculate the bearing capacity factors using Meyerhof's method
std::unordered_map<std::string, double> meyerhof(double phi) {
    std::unordered_map<std::string, double> factors;

    double Nq = exp(M_PI * tan(phi * M_PI / 180.0)) * pow(tan((45.0 + (phi / 2.0)) * M_PI / 180.0), 2);

    double Nc = (phi > 0) ? ((Nq - 1) * (1 / tan(phi * M_PI / 180.0))) : 5.14;

    double Ngamma = (Nq - 1) * tan(phi * 1.4 * M_PI / 180.0);

    factors["Nc"] = Nc;
    factors["Nq"] = Nq;
    factors["Ngamma"] = Ngamma;

    return factors;
}

// Function to calculate the bearing capacity factors using Vesic's method
std::unordered_map<std::string, double> vesic(double phi) {
    std::unordered_map<std::string, double> factors;

    double Nq = exp(M_PI * tan(phi * M_PI / 180.0)) * pow(tan((45.0 + (phi / 2.0)) * M_PI / 180.0), 2);

    double Nc = (phi > 0) ? ((Nq - 1) * (1 / tan(phi * M_PI / 180.0))) : 5.14;

    double Ngamma = 2 * (Nq + 1) * tan(phi * M_PI / 180.0);

    factors["Nc"] = Nc;
    factors["Nq"] = Nq;
    factors["Ngamma"] = Ngamma;

    return factors;
}

// Function to calculate the bearing capacity factors using Hansen's method
std::unordered_map<std::string, double> hansen(double phi) {
    std::unordered_map<std::string, double> factors;

    double Nq = exp(M_PI * tan(phi * M_PI / 180.0)) * pow(tan((45.0 + (phi / 2.0)) * M_PI / 180.0), 2);

    double Nc = (phi > 0) ? ((Nq - 1) * (1 / tan(phi * M_PI / 180.0))) : 5.14;

    double Ngamma = 1.5 * (Nq - 1) * tan(phi * M_PI / 180.0);

    factors["Nc"] = Nc;
    factors["Nq"] = Nq;
    factors["Ngamma"] = Ngamma;

    return factors;
}

// Function to calculate the bearing capacity factors following Eurocode 7 (EC7)
std::unordered_map<std::string, double> EC7(double phi) {
    std::unordered_map<std::string, double> factors;

    double Nq = exp(M_PI * tan(phi * M_PI / 180.0)) * pow(tan((45.0 + (phi / 2.0)) * M_PI / 180.0), 2);

    double Nc = (phi > 0) ? ((Nq - 1) * (1 / tan(phi * M_PI / 180.0))) : 5.14;

    double Ngamma = 2 * (Nq - 1) * tan(phi * M_PI / 180.0);

    factors["Nc"] = Nc;
    factors["Nq"] = Nq;
    factors["Ngamma"] = Ngamma;

    return factors;
}
