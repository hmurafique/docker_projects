var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/", () => new { msg = "Hello from .NET backend ⚡" });

app.Run();
