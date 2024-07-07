@echo off
setlocal

REM Check if any arguments were passed
if "%~1"=="" goto no_argument
if "%~1"=="activate" goto activate
if "%~1"=="a" goto activate
if "%~1"=="deactivate" goto deactivate
if "%~1"=="d" goto deactivate

REM If the argument is not recognized, run core.exe with the passed arguments
goto run_core

:activate
REM Activate the Python environment
call .\env\Scripts\activate
cmd /k
goto end

:deactivate
REM Deactivate the active environment
@REM call deactivate
exit
goto end

:no_argument
REM If no arguments were provided, run core.exe
:run_core
C:\Users\henri\.pen\core.exe %*
goto end

:end
endlocal