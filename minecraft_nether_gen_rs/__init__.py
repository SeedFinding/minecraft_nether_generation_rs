from . import _native


def test():
    nether_gen = _native.lib.create_new_nether(1)
    print(nether_gen)