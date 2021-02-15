# python3 setup.py install --user

from minecraft_nether_gen_rs import NetherGen,create_new_nether,get_biome,NetherBiomes
from ctypes import *
nether_gen:POINTER(NetherGen)=create_new_nether(1)
assert get_biome(nether_gen,0,0,0)==NetherBiomes.NetherWastes
print("That's a win again")