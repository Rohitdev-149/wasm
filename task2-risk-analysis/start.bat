@echo off
echo Starting local server on http://localhost:8080/frontend
start http://localhost:8080/frontend
python -m http.server 8080
pause
