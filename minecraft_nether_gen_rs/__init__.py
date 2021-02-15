from . import _native


def test():
    nether_gen = _native.lib.new(1)
    print(nether_gen)