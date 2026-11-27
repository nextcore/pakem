param(
    [string]$BaseUrl = "http://127.0.0.1:8080",
    [int]$Connections = 64,
    [string]$Duration = "10s"
)

$oha = Get-Command oha -ErrorAction SilentlyContinue
$wrk = Get-Command wrk -ErrorAction SilentlyContinue

Write-Host "=== Pakem Reproducible Benchmark ===" -ForegroundColor Cyan
Write-Host "Target: $BaseUrl | Connections: $Connections | Duration: $Duration"

$endpoints = @("/plaintext", "/json", "/db-users")

foreach ($ep in $endpoints) {
    $url = "$BaseUrl$ep"
    Write-Host "`n--- Endpoint: $ep ---" -ForegroundColor Yellow
    if ($oha) {
        & oha -z $Duration -c $Connections --no-tui $url
    } elseif ($wrk) {
        & wrk -t4 -c$Connections -d$Duration $url
    } else {
        Write-Warning "Neither 'oha' nor 'wrk' found. Install with: cargo install oha"
        $resp = Invoke-WebRequest -Uri $url -UseBasicParsing
        Write-Host "Smoke check $url -> HTTP $($resp.StatusCode) ($($resp.Content.Length) bytes)"
    }
}
