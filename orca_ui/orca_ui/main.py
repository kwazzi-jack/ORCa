import h5py
import numpy as np
from matplotlib import pyplot as plt

f = h5py.File("../../orca_engine/data.h5", "r")
true_data = np.array(f["data"]["output_data"])
noisy_data = np.array(f["data"]["noisy_data"])
pre_data = np.array(f["data"]["mp"])
fil_data = np.array(f["data"]["mf"])
smo_data = np.array(f["data"]["ms"])

x = np.arange(true_data.size)

f.close()

plt.plot(x, true_data, "k-", lw=2.0, label="True")
plt.plot(x, pre_data, "r:", label="Predictor")
plt.plot(x, fil_data, "g--", label="Filter")
plt.plot(x, smo_data, "b-", label="Smoother")
plt.xlabel("Time")
plt.ylabel("Value")
plt.legend()
plt.title("Simulated Kalman Filter and Smoother")
plt.show()
