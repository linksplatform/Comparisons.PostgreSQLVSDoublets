#include <Platform.Data.Doublets.h>
#include <benchmark/benchmark.h>
#include <Client.h>
#include <Transaction.h>
#include <PostgreSQL.h>

const std::string options = "user=postgres dbname=postgres password=postgres host=localhost port=5432";
const std::uint64_t BACKGROUND_LINKS = 3000;

#include "SetupTeardown.cpp"
#include "Create.cpp"
#include "Delete.cpp"
#include "Each/Identity.cpp"
#include "Each/Concrete.cpp"
#include "Each/Outgoing.cpp"
#include "Each/Incoming.cpp"
#include "Each/All.cpp"
#include "Update.cpp"

BENCHMARK_MAIN();
