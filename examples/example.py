# python3 setup.py install --user

from minecraft_nether_gen_rs import NetherGen,create_new_nether,get_biome,NetherBiomes,delete
from ctypes import *
nether_gen:POINTER(NetherGen)=create_new_nether(1)
assert get_biome(nether_gen,0,0,0)==NetherBiomes.NetherWastes
print("That's a win again")
delete(nether_gen)
# Requires Python3.8
from importlib.metadata import version
print(version("minecraft_nether_gen_rs"))


"""
It's possible to install for python2, but beware that you will need to modify ../minecraft_nether_gen_rs/__init__.py
to remove all the type hinting.
We recommend python3.6+ as it is more recent.
"""