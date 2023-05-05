#include <Platform.Data.Doublets.h>
#include <benchmark/benchmark.h>
#include <Client.h>
#include <Transaction.h>

const std::string options = "";

#include "SetupTeardown.cpp"
#include "CreatePoints.cpp"
#include "UpdateLinks.cpp"
#include "DeleteLinks.cpp"
#include "EachLink.cpp"

BENCHMARK_MAIN();
