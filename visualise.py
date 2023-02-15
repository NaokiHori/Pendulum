import os
import sys
import numpy as np
from matplotlib import pyplot
from matplotlib import patches


def visualise(ax, data):
    radius = 0.25
    color = "#FF0000"
    # draw lines connecting each mass incl. origin
    xs = np.append(0., data[0, :])
    ys = np.append(0., data[1, :])
    size = len(xs) + radius
    ax.plot(xs, ys, color="#FFFFFF", linestyle="solid", linewidth=0.5, zorder=0)
    # draw circles for each mass (no origin here)
    for x, y in zip(xs[1:], ys[1:]):
        c = patches.Circle(xy=(x, y), radius=radius, edgecolor=color, facecolor=color)
        ax.add_patch(c)
    # configuration
    keywords = {
            "facecolor": "#000000",
            "title": "",
            "aspect": "equal",
            "xlim": [-1.0 * size, +1.0 * size],
            "ylim": [-1.0 * size, +0.5 * size],
            "xlabel": "",
            "ylabel": "",
            "xticks": [],
            "yticks": [],
    }
    ax.set(**keywords)

def main():
    # load pendulum positions
    path_output = "output"
    prefix = "mass"
    suffix = ".dat"
    fnames = [f"{path_output}/{fname}" for fname in os.listdir(path_output) if fname.startswith(prefix) and fname.endswith(suffix)]
    fnames = sorted(fnames)
    if 0 == len(fnames):
        return
    # expand all data to memory
    data = list()
    for fname in fnames:
        datum = np.loadtxt(fname, usecols=[1, 2])
        data.append(datum)
    data = np.array(data)
    # re-order, so that results in
    #   "for each time, for each coordinate, for each mass"
    data = np.transpose(data, [1, 2, 0])
    # create axes instance
    fig = pyplot.figure(figsize=(8, 6))
    ax = fig.add_subplot(111)
    for datum in data:
        # clean-up canvas
        ax.clear()
        # draw objects
        visualise(ax, datum)
        pyplot.show(block=False)
        pyplot.pause(1.e-8)
    pyplot.close()


if __name__ == "__main__":
    main()

