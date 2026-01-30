Solution overview and reproduction steps

Projects:
- LibraryA (class library) - references SQLitePCLRaw.bundle_e_sqlite3
- ProcessA (console app) - references LibraryA but does NOT reference SQLite packages (intentional)
- TestProj (xUnit) - launches ProcessA.dll from test bin working directory

Repro steps (manual):
1. dotnet restore
2. dotnet build
3. Publish LibraryA to ensure native e_sqlite3.dll is available: dotnet publish -c Release -r win-x64 -o publish\lib
   (or rely on NuGet package assets in packages folder)
4. dotnet publish ProcessA -c Release -r win-x64 -o publish\processA
5. Copy native sqlite dll (e_sqlite3.dll) into TestProj bin folder (where tests will run) and copy ProcessA.dll next to it.
6. Run tests: dotnet test TestProj

Expected outcome:
- Even with native e_sqlite3.dll in TestProj/bin, ProcessA run via dotnet ProcessA.dll from that working directory will fail because ProcessA.deps.json lacks sqlite entries (ProcessA has no package reference). The error will indicate inability to load or initialize native sqlite (dll not found or initialization failure).

Fix:
- Add the same PackageReference (SQLitePCLRaw.bundle_e_sqlite3) to ProcessA.csproj. That ensures dependencies are included into ProcessA.deps.json and runtime will load native dlls correctly.
