


def _mul(x: int, y: int, d: int) -> int:
    return (x * y) / _scale(d)

def _scale(d: int) -> int:
    return 10**d

print(10**0)