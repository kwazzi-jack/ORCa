import h5py
import numpy as np
from matplotlib import pyplot as plt

f = h5py.File("../../orca_engine/data.h5", "r")
y1 = np.array(f["data"]["output_data"])
y2 = np.array(f["data"]["noisy_data"])
x = np.arange(y1.size)

f.close()

plt.plot(x, y1, "k-", label="True")
plt.plot(x, y2, "r:", label="Noisy")
plt.xlabel("Time")
plt.ylabel("Value")
plt.legend()
plt.title("Simulated Output Data")
plt.show()
