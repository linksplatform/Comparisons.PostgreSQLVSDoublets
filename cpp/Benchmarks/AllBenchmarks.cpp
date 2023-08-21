#include <Platform.Data.Doublets.h>
#include <Platform.Data.h>
#include <pqxx/pqxx>
#include <benchmark/benchmark.h>
#include <ISQL.h>
#include <PSQLBase.h>
#include <Client.h>
#include <Transaction.h>

const std::string options {"user=postgres dbname=postgres password=postgres host=localhost port=5432"};
constexpr std::uint64_t BACKGROUND_LINKS {3000};

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
