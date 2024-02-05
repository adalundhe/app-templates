app := "myapp.main:app"
host := "0.0.0.0"
port := "5000"

init username email:
    git config --global --add safe.directory /workspace
    git config --global user.email "{{email}}"
    git config --global user.name "{{username}}"

dev app=app host=host port=port:
    uvicorn {{app}} --host {{host}} --port {{port}} --reload

commit message branch:
    git add -A
    git commit -m '{{message}}'
    git push origin {{branch}}