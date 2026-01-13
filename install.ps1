# B58UUID CLI Installation Script for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/b58uuid/b58uuid-cli/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# Configuration
$Repo = "b58uuid/b58uuid-cli"
$BinaryName = "b58uuid"
$InstallDir = "$env:USERPROFILE\.b58uuid"

# Colors
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# Get latest version
function Get-LatestVersion {
    Write-ColorOutput Yellow "Fetching latest version..."
    
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $version = $response.tag_name
        Write-ColorOutput Green "Latest version: $version"
        return $version
    }
    catch {
        Write-ColorOutput Red "Error: Failed to fetch latest version"
        exit 1
    }
}

# Download and install
function Install-Binary {
    param($Version)
    
    $Platform = "windows-amd64"
    $DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$BinaryName-$Platform.zip"
    $TempDir = New-Item -ItemType Directory -Path "$env:TEMP\b58uuid-install-$(Get-Random)" -Force
    $ZipFile = Join-Path $TempDir "$BinaryName.zip"
    
    Write-ColorOutput Yellow "Downloading $BinaryName $Version for $Platform..."
    
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipFile
    }
    catch {
        Write-ColorOutput Red "Error: Failed to download $DownloadUrl"
        Remove-Item -Recurse -Force $TempDir
        exit 1
    }
    
    Write-ColorOutput Yellow "Extracting..."
    Expand-Archive -Path $ZipFile -DestinationPath $TempDir -Force
    
    Write-ColorOutput Yellow "Installing to $InstallDir..."
    
    # Create install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Copy binary
    $BinaryPath = Join-Path $TempDir "$BinaryName.exe"
    $DestPath = Join-Path $InstallDir "$BinaryName.exe"
    Copy-Item -Path $BinaryPath -Destination $DestPath -Force
    
    # Clean up
    Remove-Item -Recurse -Force $TempDir
    
    Write-ColorOutput Green "✓ $BinaryName installed successfully!"
    
    return $DestPath
}

# Add to PATH
function Add-ToPath {
    param($Dir)
    
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($UserPath -notlike "*$Dir*") {
        Write-ColorOutput Yellow "Adding $Dir to PATH..."
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$UserPath;$Dir",
            "User"
        )
        $env:Path = "$env:Path;$Dir"
        Write-ColorOutput Green "✓ Added to PATH"
    }
    else {
        Write-ColorOutput Green "✓ Already in PATH"
    }
}

# Verify installation
function Test-Installation {
    param($BinaryPath)
    
    if (Test-Path $BinaryPath) {
        try {
            $version = & $BinaryPath --version
            Write-ColorOutput Green "✓ Installation verified: $version"
            return $true
        }
        catch {
            Write-ColorOutput Yellow "Warning: Binary installed but cannot execute"
            return $false
        }
    }
    else {
        Write-ColorOutput Red "Error: Binary not found at $BinaryPath"
        return $false
    }
}

# Main
function Main {
    Write-ColorOutput Green "B58UUID CLI Installer"
    Write-Output ""
    
    $version = Get-LatestVersion
    $binaryPath = Install-Binary -Version $version
    Add-ToPath -Dir $InstallDir
    $success = Test-Installation -BinaryPath $binaryPath
    
    Write-Output ""
    Write-ColorOutput Green "Installation complete!"
    Write-Output ""
    
    if ($success) {
        Write-Output "Try it out:"
        Write-Output "  $BinaryName encode 550e8400-e29b-41d4-a716-446655440000"
        Write-Output "  $BinaryName generate"
        Write-Output "  $BinaryName --help"
    }
    else {
        Write-ColorOutput Yellow "Please restart your terminal and try again."
    }
    
    Write-Output ""
    Write-Output "For more information, visit: https://b58uuid.io"
}

Main
