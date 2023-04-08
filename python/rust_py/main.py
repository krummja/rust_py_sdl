from rustpy.rustpy import sum_as_string
from rustpy.rustpy import ColorWrapper
from rustpy.rustpy import create_context
from rich import inspect


if __name__ == '__main__':
    print(sum_as_string(1, 2))
    color = ColorWrapper(r=10, g=20, b=255, a=1)
    inspect(color, all=True)

    create_context()
