# Variables
$pythonInstallerUrl = "https://www.python.org/ftp/python/3.10.6/python-3.10.6-amd64.exe"
$tempDirectory = "C:\temp_provision\"
$targetDir = "C:\Python3_10_6"
$pythonInstallerPath = $tempDirectory + "python3_10_6.exe"

# Create temporary directory if it doesn't exist
New-Item -ItemType directory -Path $tempDirectory -Force | Out-Null

# Download Python installer
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
(New-Object System.Net.WebClient).DownloadFile($pythonInstallerUrl, $pythonInstallerPath)

# Create target directory if it doesn't exist
New-Item -ItemType directory -Path $targetDir -Force | Out-Null

# Extract Python files
Start-Process -FilePath $pythonInstallerPath -ArgumentList @("/quiet", "TargetDir=$targetDir") -Wait

# Clean up
Remove-Item $pythonInstallerPath

Write-Host "Python 3.10.6 has been downloaded and extracted to $targetDir"