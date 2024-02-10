from werkflow import (
    Workflow,
    step
)


class {{ cookiecutter.app.replace('-', '_').split('_') | map('capitalize') | join('') }}(Workflow):

    @step()
    async def hello_world(self):
        await self.shell.run(
            'echo "Hello world!"'
        )