#include <Platform.Data.Doublets.h>
#include <benchmark/benchmark.h>
#include <Client.h>
#include <Transaction.h>

const std::string options = "user=postgres dbname=postgres password=postgres host=localhost port=5432";
const std::uint64_t BACKGROUND_LINKS = 3000;

#include "SetupTeardown.cpp"
#include "CreatePoints.cpp"
#include "DeleteLinks.cpp"
#include "EachLink.cpp"
#include "UpdateLinks.cpp"

BENCHMARK_MAIN();
