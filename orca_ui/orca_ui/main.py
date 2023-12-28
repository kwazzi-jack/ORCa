import h5py
from matplotlib import pyplot

f = h5py.File("data.h5", "r")
list(f.keys())
