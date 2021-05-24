import os
import matplotlib.pyplot as plt
import json


RAND_MILLION = "rand-1000000"
HG38_MILLION = "hg38-1000000"
READS_S = "reads-100-10-1"
READS_L = "reads-1000-200-1"

EXE_PATH = os.path.join("target", "release", "gene_search.exe")
SPACINGS = [1, 2, 3, 4, 5, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32]
GENOME_SIZES = [200000, 400000, 600000, 800000, 1000000, 5000000, 10000000, 50000000]
HG38_SIZES = [1000, 10000, 100000, 1000000]

"""
sais genome iterations
naive-sa genome iterations
otable genome iterations skips
approx genome reads iterations skips
exact genome reads iterations skips
"""


def main():
    initialize()
    # ex1()
    # ex2()
    # ex4()
    # ex5()
    # ex7()
    # ex8()
    # ex9()
    # ex10()
    # ex11()
    ex12()
    ex13()


def ex1():
    """Eksperiment 1

    Hvilken effekt har det at fjerne sentinel row fra o-table på construction tid?

    Køres på:
    - main
    - otable-nosentinelrow
    - otable-nosentinelrow-branchlessconstruction
    """
    graph_variable_genome("ex1", "otable", 50, spacing=1)


def ex2():
    """Eksperiment 2

    Hvilken effekt har det at fjerne sentinel row fra o-table på access tid?

    Køres på:
    - main
    - otable-nosentinelrow
    """
    graph_variable_genome("ex2-approx", "approx", 50, reads=READS_S, spacing=1, edits=1)
    graph_variable_genome("ex2-exact-bwt", "exact-bwt", 500, reads=READS_L, spacing=1)


def ex4():
    """Eksperiment 4

    Hvilken effekt har skips på construction tid?

    Køres på:
    - main
    - otable-nosentinelrow
    - otable-nosentinelrow-branchlessconstruction
    """
    graph_variable_spacing("ex4", "otable", RAND_MILLION, 50)


def ex5():
    """Eksperiment 5

    Hvilken effekt har skips på access tid?

    Køres på:
    - main
    - otable-nosentinelrow
    """
    graph_variable_spacing("ex5-approx", "approx", RAND_MILLION, 50, reads=READS_S, edits=1)
    graph_variable_spacing("ex5-exact-bwt", "exact-bwt", RAND_MILLION, 500, reads=READS_L)


def ex7():
    """Eksperiment 7

    Hvor stort overhead introducerer kapacitet for skips til konstruction?

    Køres på:
    - main
    - otable-noskips
    """
    graph_variable_genome("ex7", "otable", 50, spacing=1)


def ex8():
    """Eksperiment 8

    Hvor stort overhead introducerer kapacitet for skips til access?

    Køres på:
    - main
    - otable-noskips
    """
    graph_variable_genome("ex8-approx", "approx", 50, reads=READS_S, spacing=1, edits=1)
    graph_variable_genome("ex8-exact-bwt", "exact-bwt", 500, reads=READS_L, spacing=1)


def ex9():
    """Eksperiment 9

    Hvilken effekt har branchless på SA-IS?

    Køres på:
    - main
    - sais-branchlessinduce
    """
    graph_variable_genome("ex9", "sais", 5)


def ex10():
    """Eksperiment 10

    Hvor hurtig er SA-IS i forhold til naiv og skew?

    Køres på:
    - main
    """
    graph_variable_genome("ex10-sais", "sais", 5, spacing=1)
    graph_variable_genome("ex10-naive-sa", "naive-sa", 5, spacing=1)
    graph_variable_genome("ex10-skew", "skew", 5, spacing=1)


def ex11():
    """Eksperiment 11

    Hvor hurtig er BWT-search ift. binary search?

    Køres på:
    - main
    """
    graph_variable_genome("ex11-exact-bwt", "exact-bwt", 100, reads=READS_L, spacing=1)
    graph_variable_genome("ex11-exact-binary", "exact-binary", 100, reads=READS_L, spacing=1)


def ex12():
    """Eksperiment 12

    Hvor meget påvirker flere edits køretiden af approx?

    Køres på:
    - main
    """
    def g(*xs):
        for x in xs:
            graph_variable_genome_hg38(f"ex12-approx-{x}", "approx", 10, reads=READS_S, spacing=1, edits=x)

    g(0, 1, 2, 3, 4, 5)


def ex13():
    """Eksperiment 13

    Hvor meget påvirker flere edits køretiden af approx? BIG EDITION

    Køres på:
    - main
    """
    def g(*xs):
        for x in xs:
            graph_variable_genome_hg38(f"ex13-approx-{x}", "approx", 10, reads="reads-100-100-2", spacing=1, edits=x)

    g(0, 10, 20, 30, 40, 50)


def initialize():
    build = "cargo build --release --quiet"
    os.popen(build).read()


def graph_variable_spacing(fname, type, genome, iterations, reads=None):
    reads = "" if reads is None else f"{reads} "

    print(f"Computing {fname}...")
    git_branch = os.popen("git branch --show-current").read().strip()

    data = []

    x = []  # skip sizes
    y = []  # nanos

    for spacing in SPACINGS:
        print(f"{spacing:<3}", end='')
    print(flush=True)

    for spacing in SPACINGS:
        print("\u2588" * 3, end="", flush=True)

        res = os.popen(f"{EXE_PATH} {type} {genome} {reads}{iterations} {spacing} --no-output").read()
        average_ns = int(res)

        data.append({
            "spacing": spacing,
            "average_nanoseconds": average_ns,
        })

        x.append(spacing)
        y.append(average_ns)

    print(flush=True)

    path = os.path.join("results", "graphs", git_branch)
    if not os.path.exists(path):
        os.makedirs(path)

    # Save data to json file
    data_file = os.path.join(path, f"{fname}.json")
    with open(data_file, 'w') as f:
        json.dump(data, f)

    # Make pyplot
    plt.xlabel("Skip size")
    plt.ylabel("Nanoseconds")
    plt.grid()
    plt.plot(x, y)
    figure_location = os.path.join(path, f"{fname}.pdf")
    plt.savefig(figure_location)
    plt.close()


def graph_variable_genome(fname, type, iterations, reads=None, spacing=None, edits=None):
    iterations = "" if iterations is None else f"{iterations} "
    reads = "" if reads is None else f"{reads} "
    spacing = "" if spacing is None else f"{spacing} "
    edits = "" if edits is None else f"{edits}"

    print(f"Computing {fname}...")
    git_branch = os.popen("git branch --show-current").read().strip()

    data = []

    x = []  # genome lengths
    y = []  # nanos

    for genome_size in GENOME_SIZES:
        print(f"{genome_size:<8}", end='')
    print(flush=True)

    for genome_size in GENOME_SIZES:
        print("\u2588" * 8, end="", flush=True)

        genome = f"rand-{genome_size}"

        res = os.popen(f"{EXE_PATH} {type} {genome} {reads}{iterations}{spacing}{edits} --no-output").read()
        average_ns = int(res)

        data.append({
            "genome_size": genome_size,
            "average_nanoseconds": average_ns,
        })

        x.append(genome_size)
        y.append(average_ns)

    print(flush=True)

    path = os.path.join("results", "graphs", git_branch)
    if not os.path.exists(path):
        os.makedirs(path)

    # Save data to json file
    data_file = os.path.join(path, f"{fname}.json")
    with open(data_file, 'w') as f:
        json.dump(data, f)

    # Make pyplot
    plt.xlabel("Genome length")
    plt.ylabel("Nanoseconds")
    plt.grid()
    plt.plot(x, y)
    figure_location = os.path.join(path, f"{fname}.pdf")
    plt.savefig(figure_location)
    plt.close()


def graph_variable_genome_hg38(fname, type, iterations, reads=None, spacing=None, edits=None):
    iterations = "" if iterations is None else f"{iterations} "
    reads = "" if reads is None else f"{reads} "
    spacing = "" if spacing is None else f"{spacing} "
    edits = "" if edits is None else f"{edits}"

    print(f"Computing {fname}...")
    git_branch = os.popen("git branch --show-current").read().strip()

    data = []

    x = []  # genome lengths
    y = []  # nanos

    for genome_size in HG38_SIZES:
        print(f"{genome_size:<8}", end='')
    print(flush=True)

    for genome_size in HG38_SIZES:
        print("\u2588" * 8, end="", flush=True)

        genome = f"hg38-{genome_size}"

        res = os.popen(f"{EXE_PATH} {type} {genome} {reads}{iterations}{spacing}{edits} --no-output").read()
        average_ns = int(res)

        data.append({
            "genome_size": genome_size,
            "average_nanoseconds": average_ns,
        })

        x.append(genome_size)
        y.append(average_ns)

    print(flush=True)

    path = os.path.join("results", "graphs", git_branch)
    if not os.path.exists(path):
        os.makedirs(path)

    # Save data to json file
    data_file = os.path.join(path, f"{fname}.json")
    with open(data_file, 'w') as f:
        json.dump(data, f)

    # Make pyplot
    plt.xlabel("Genome length")
    plt.ylabel("Nanoseconds")
    plt.grid()
    plt.plot(x, y)
    figure_location = os.path.join(path, f"{fname}.pdf")
    plt.savefig(figure_location)
    plt.close()


if __name__ == '__main__':
    main()
