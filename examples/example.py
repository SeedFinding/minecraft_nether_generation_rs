# python3 setup.py install --user

from minecraft_nether_gen_rs import NetherGen,create_new_nether,get_biome,NetherBiomes,delete,get_biome_structure,get_biome_decorator
from ctypes import *

nether_gen:POINTER(NetherGen)=create_new_nether(1)
assert get_biome(nether_gen,0,0,0)==NetherBiomes.NetherWastes
delete(nether_gen)

chunk_x=-3
chunk_z=0
nether_gen:POINTER(NetherGen)=create_new_nether(5772394932603802938)
assert get_biome_structure(nether_gen,chunk_x,chunk_z)==NetherBiomes.SoulSandValley
assert get_biome_decorator(nether_gen,chunk_x,chunk_z)==NetherBiomes.SoulSandValley
delete(nether_gen)
print("That's a win again")

# Requires Python3.8
from importlib.metadata import version
print(version("minecraft_nether_gen_rs"))


"""
It's possible to install for python2, but beware that you will need to modify ../minecraft_nether_gen_rs/__init__.py
to remove all the type hinting.
We recommend python3.6+ as it is more recent.
"""