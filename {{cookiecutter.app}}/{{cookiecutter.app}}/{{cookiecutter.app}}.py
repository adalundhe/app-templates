from werkflow import (
    Workflow,
    step
)


class {{ cookiecutter.app | capitalize | replace('-', '') | replace('_', '')}}(Workflow):

    @step()
    async def hello_world(self):
        print('Hello world!')