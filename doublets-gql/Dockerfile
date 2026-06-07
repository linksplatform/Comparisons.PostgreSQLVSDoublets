# Dockerfile for Data.Doublets.Gql server
FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build

# Clone the Data.Doublets.Gql repository
RUN git clone https://github.com/linksplatform/Data.Doublets.Gql.git /src

WORKDIR /src/csharp/Platform.Data.Doublets.Gql.Server

# Restore dependencies
RUN dotnet restore

# Build the application
RUN dotnet publish -c Release -o /app

# Runtime image
FROM mcr.microsoft.com/dotnet/aspnet:6.0
WORKDIR /app
COPY --from=build /app .

# Create directory for database
RUN mkdir -p /data

# Expose the GraphQL server port
EXPOSE 60341

# Run the server with database in /data volume
ENTRYPOINT ["dotnet", "Platform.Data.Doublets.Gql.Server.dll", "/data/doublets.links", "--urls", "http://0.0.0.0:60341"]