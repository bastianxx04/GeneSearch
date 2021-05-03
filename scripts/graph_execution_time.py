import os
import matplotlib.pyplot as plt
import json


def main():
    initialize()
    graph_with_spacing("exact", "otable", "approx")


def initialize():
    build = "cargo build --release --quiet"
    os.popen(build).read()


def graph_with_spacing(*args):
    run = os.path.join("target", "release", "gene_search.exe")
    git_branch = os.popen("git branch --show-current").read().strip()

    iterations = 5

    for parameter in args:
        print(f"Computing {parameter}...")

        data = []

        x = []  # skip sizes
        y = []  # nanos

        for spacing in [1, 2, 4, 8, 16, 32]:
            print(f"{spacing:<3}", end='', flush=True)

            res = os.popen(f"{run} {parameter} {spacing} {iterations} --no-output").read()
            average_ns = int(res)

            data.append({
                "spacing": spacing,
                "average_nanoseconds": average_ns,
            })

            y.append(average_ns)
            x.append(spacing)

        print(flush=True)

        path = os.path.join("results", "graphs", git_branch)
        if not os.path.exists(path):
            os.makedirs(path)

        # Save data to json file
        data_file = os.path.join(path, f"{parameter}.json")
        with open(data_file, 'w') as f:
            json.dump(data, f)

        # Make pyplot
        plt.xlabel("Skip size")
        plt.ylabel("Nanoseconds")
        plt.grid()
        plt.plot(x, y)
        figure_location = os.path.join(path, f"{parameter}.pdf")
        plt.savefig(figure_location)
        plt.close()


if __name__ == '__main__':
    main()
