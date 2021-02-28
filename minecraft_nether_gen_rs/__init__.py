from . import _native
from ctypes import *
from enum import IntEnum


class CtypesEnum(IntEnum):
    """A ctypes-compatible IntEnum superclass."""

    @classmethod
    def from_param(cls, obj):
        return int(obj)


class NetherBiomes(CtypesEnum):
    NetherWastes = 8,
    TheVoid = 127,
    SoulSandValley = 170,
    CrimsonForest = 171,
    WarpedForest = 172,
    BasaltDeltas = 173,


class NetherGen(Structure):
    _pack_ = 8
    _fields_ = [("seed", c_uint64),
                ("_noise", c_void_p),
                ("is_3d", c_bool)]


def create_new_nether(seed) -> POINTER(NetherGen):
    return _native.lib.create_new_nether(seed)


def get_biome(nether_gen: POINTER(NetherGen), x: c_int32, y: c_int32, z: c_int32) -> NetherBiomes:
    return _native.lib.get_biome(nether_gen, x, y, z)


def get_biome_structure(nether_gen: POINTER(NetherGen), chunk_x: c_int32, chunk_z: c_int32) -> NetherBiomes:
    return _native.lib.get_biome_structure(nether_gen, chunk_x, chunk_z)


def get_biome_decorator(nether_gen: POINTER(NetherGen), chunk_x: c_int32, chunk_z: c_int32) -> NetherBiomes:
    return _native.lib.get_biome_decorator(nether_gen, chunk_x, chunk_z)


def delete(nether_gen: POINTER(NetherGen)):
    return _native.lib.delete(nether_gen)
