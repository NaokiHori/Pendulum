import sys
import numpy
import matplotlib
matplotlib.use("Agg")
from matplotlib import pyplot

def energy1(ofname, xs, y1s, y2s, y3s):
    fig = pyplot.figure()
    ax = fig.add_subplot(111)
    ax.plot(xs, y1s, label="T", color="#ff0000")
    ax.plot(xs, y2s, label="U", color="#0000ff")
    ax.plot(xs, y3s, label="E", color="#000000")
    ax.set(
            xlabel="time",
            ylabel="energy",
            xlim=[0, 100],
    )
    pyplot.legend()
    pyplot.savefig(ofname)
    pyplot.close()

def energy2(ofname, xs, ys):
    fig = pyplot.figure()
    ax = fig.add_subplot(111)
    ax.plot(xs, ys, label="E", color="#000000")
    ax.set(
            xlabel="time",
            ylabel="energy",
            xlim=[0, 1000],
    )
    pyplot.legend()
    pyplot.savefig(ofname)
    pyplot.close()

def main(ifname, ofnames):
    data = numpy.loadtxt(ifname)
    xs = data[:, 0]
    y1s = data[:, 1]
    y2s = data[:, 2]
    y3s = data[:, 3]
    energy1(ofnames[0], xs, y1s, y2s, y3s)
    energy2(ofnames[1], xs, y3s)

if __name__ == "__main__":
    argv = sys.argv
    assert 4 == len(argv)
    main(argv[1], (argv[2], argv[3]))

